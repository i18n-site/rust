export PID=$$
export SCRIPT_NAME=$(realpath $0)
RED='\033[0;31m'
END='\033[0m'

lock="/tmp/${SCRIPT_NAME//\//_}.pid"
mkdir -p $(dirname $lock)

if [[ -f "$lock" ]]; then
  pid=$(cat $lock)
  while kill $pid >/dev/null 2>&1; do
    echo "$SCRIPT_NAME: pid $pid running , trying kill it"
    sleep 1
    echo -e "$RED$ kill -9 $pid$END"
    kill -9 $pid || true
  done
fi

trap "rm -f $lock; exit $?" INT TERM EXIT
echo $PID >$lock
