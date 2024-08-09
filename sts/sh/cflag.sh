#!/usr/bin/env bash

if ! [ -x "$(command -v cargo)" ]; then
  cargo_env="$HOME/.cargo/env"
  if [ -f "$cargo_env" ]; then
    source $cargo_env
  fi
fi

RUSTFLAGS=" --cfg reqwest_unstable"
CFLAGS+=" -O3"

if [[ $(uname -s) == Linux ]]; then
  if ! [ -x "$(command -v nasm)" ]; then
    apt install -y nasm
  fi
fi

if [ -z "$NATIVE" ]; then
  if [[ $(uname -s) == Linux && $(uname -m) == x86_64 ]]; then
    # Intel:
    # sse3 pclmulqdq ssse3 fma cmpxchg16b pcid sse41 sse42 x2apic movbe popcnt aesni xsave oxsave avx f16c rdrand fpu vme de pse tsc msr pae mce cmpxchg8b apic sysenter_sysexit mtrr pge mca cmov pat pse36 clflush mmx fxsave_fxstor sse sse2 ss htt bmi1 hle avx2 fdp smep bmi2 rep_movsb_stosb invpcid rtm fpu_cs_ds_deprecated mpx rdseed adx smap clflushopt avx512f avx512dq avx512cd avx512bw avx512vl clwb
    #
    # AMD:
    # fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl nonstop_tsc cpuid extd_apicid tsc_known_freq pni pclmulqdq ssse3 fma cx16 sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm cmp_legacy cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw topoext ssbd ibrs ibpb stibp vmmcall fsgsbase tsc_adjust bmi1 avx2 smep bmi2 rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 clzero xsaveerptr arat npt nrip_save umip rdpid

    # avx512bw avx512cd avx512dq avx512vl
    export RUST_FEATURES=" --features stackdriver"
    RUSTFLAGS+=" -C target-feature="

    FEATURES="sse sse2 ssse3 sse4 fma avx avx2 pclmulqdq popcnt xsave mmx"
    RUST_IGNORE="sse4 clflush mmx"
    C_IGNORE="pclmulqdq"
    for flag in $FEATURES; do
      if [[ ! " $C_IGNORE " =~ " $flag " ]]; then
        CFLAGS+=" -m$flag"
      fi
      CFLAGS+=" -mpclmul"
      if [[ ! " $RUST_IGNORE " =~ " $flag " ]]; then
        RUSTFLAGS+="+$flag,"
      fi
    done
    RUSTFLAGS=${RUSTFLAGS%?}
  fi
else
  CFLAGS+=" -march=native"
  RUSTFLAGS+=" -C target-cpu=native"
fi

export CFLAGS=$CFLAGS
export CXXFLAGS=$CFLAGS
export RUSTFLAGS=$RUSTFLAGS
RUST_TARGET=$(rustc -vV | grep "host:" | awk '{print $2}')
