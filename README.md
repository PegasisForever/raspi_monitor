# Raspberry Pi Monitor

Generate a json formatted system status. Used in [Raspberry Pi Monitor App](https://github.com/PegasisForever/raspi_monitor_app).

## Sample Output

```
{"v":1,"time":1593492355861,"cpu_temp":43.470001220703125,"mem_used_kb":197264,"mem_total_kb":948280,"load_1":0.20999999344348907,"load_5":0.05999999865889549,"load_15":0.019999999552965164,"cpu_idle_time":486335616,"cpu_total_time":487843553,"cpu_mhz":585.9375,"cpu_min_mhz":585.9375,"cpu_max_mhz":1171.875,"received_bytes":937589833,"sent_bytes":1605698763,"swap_total_kb":102396,"swap_used_kb":256,"total_disk_read_kb":431892.5,"total_disk_write_kb":1479665,"root_used_kb":3307704,"root_total_kb":61089604}
```

## Performance

Takes around 0.010s to run on my raspberry pi 3b.

## Compile Release

```
./build.sh
```

gzipped binary for arm32, arm64, x86 and x64 will be in `target/deploy`

