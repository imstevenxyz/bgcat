export BGCAT_DEBUG=true;
export BGCAT_LOG_LEVEL="debug";

cargo watch --quiet \
    --ignore container \
    --ignore docs \
    --ignore dev.sh \
    --ignore LICENSE \
    --ignore README.adoc \
    --exec run