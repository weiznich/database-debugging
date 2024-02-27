# Database Debugging

Repository for debugging large inserts via `diesel`. This setup reads all of the rows into memory, chunks them, then pushes them into `postgres` with `rayon`, which takes 3-4 minutes. 

Another setup tested pushed rows into the array as they were generated, in batches of 10922(Max parameter limit of 65535, divided by 6 parameters per row). This acheived a runtime of around 6 minutes.

To test this project, you should do the following:

1. Download the dataset:
```sh
curl https://datasets.imdbws.com/title.principals.tsv.gz | gunzip - > data/title.principals.tsv
```
2. Start/initialize the database:
```sh
$ docker-compose up -d
# ...
$ diesel setup
```
3. Run the test:
```sh
$ cargo run
```