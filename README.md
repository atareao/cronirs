# cronirs

This is a cron for *Docker* developed in Rust.

## Crontab

This service uses [tokio-crate-scheduler](https://docs.rs/crate/tokio-cron-scheduler/latest). You must set a *crontab.txt* with the following format,


```cron
sec   min   hour   day of month   month   day of week   year
*     *     *      *              *       *             *

```

## environment variables

```bash
LOG_LEVEL=debug
LOCAL_USER_ID=1000
LOCAL_GROUP_ID=1000
CRONTAB=/app/crontab.txt

```

## Build

To generate the file `docker-bake.hcl` you must run this,

```bash
bash genbake.sh
```

After that,

```
bash init_build.sh
docker buildx bake --pull --push
```
