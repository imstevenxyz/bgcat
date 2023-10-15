export BGCAT_DEBUG=true;
export BGCAT_LOG_LEVEL="debug";
export BGCAT_DB_START_LOCAL="false";


surreal start --auth --user root --pass root --bind 127.0.0.1:8001 file:./data/bgcat.db &
cargo watch --quiet \
    --ignore container \
    --ignore docs \
    --ignore dev.sh \
    --ignore LICENSE \
    --ignore README.adoc \
    --exec run