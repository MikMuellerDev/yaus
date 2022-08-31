FROM alpine:3

RUN addgroup yaus && adduser \
    --disabled-password \
    --gecos "" \
    --home /app \
    --ingroup yaus \
    --no-create-home \
    --uid 1000 \
    yaus

RUN apk update \
    && apk add --no-cache ca-certificates tzdata \
    && rm -rf /var/cache/apk/*

RUN mkdir /app

COPY ./yaus /app/yaus

WORKDIR /app/

CMD ["/app/yaus"]
