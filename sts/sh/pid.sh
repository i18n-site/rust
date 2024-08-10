export PID=$$
export SCRIPT_NAME=$(realpath $0)
RED='\033[0;31m'
END='\033[0m'

lock="/tmp/${SCRIPT_NAME//\//_}.pid"

trap "set +x;rm -f $lock; exit $?" INT TERM EXIT

mkdir -p $(dirname $lock)

if [[ -f "$lock" ]]; then
  pid=$(cat $lock)
  echo -e "$RED$ kill -9 $pid$END"
  kill -9 $pid || true
  while [ -e /proc/$pid ]; do
    echo "$SCRIPT_NAME: pid $pid running , trying kill it"
    kill $pid || true
    sleep 1
    kill -9 $pid || true
  done
fi

echo $PID >$lock
