#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

[ "$UID" -eq 0 ] || exec sudo "$0" "$@"

name=$1

EXE=/opt/bin/$name

if [ ! -f "$EXE" ]; then
  ./build.native.sh
fi

service_sh=/opt/bin/$name.service.sh

cat >$service_sh <<EOF
#!/usr/bin/env bash
source $DIR/env.sh
exec $EXE
EOF

chmod +x $service_sh

system_service=/etc/systemd/system/$name.service
cp $DIR/service $system_service

sed -i "s#EXEC#${service_sh}#" $system_service
sed -i "s#NAME#${name}#" $system_service

systemctl daemon-reload

systemctl enable --now $name || true
systemctl restart $name

systemctl status $name --no-pager || true

journalctl -u $name -n 10 --no-pager --no-hostname
