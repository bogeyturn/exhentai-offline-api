CREATE table if not exists failed_reasons (
    id SERIAL not null primary key,
    reason text not null
)
