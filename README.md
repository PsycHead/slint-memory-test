# Slint high RAM usage example

Run with `cargo run --release`.

## Results on my machine (with QT)
```
########## Without window ##########
Current physical memory usage: 70 MB
Current virtual memory usage: 154 MB

########## With window ##########
Current physical memory usage: 102 MB
Current virtual memory usage: 406 MB

########## Without window (again) ##########
Current physical memory usage: 107 MB
Current virtual memory usage: 410 MB
```

## Results on my machine (with Winit)

```
########## Without window ##########
Current physical memory usage: 2 MB
Current virtual memory usage: 7 MB

########## With window ##########
Current physical memory usage: 167 MB
Current virtual memory usage: 392 MB

########## Without window (again) ##########
Current physical memory usage: 167 MB
Current virtual memory usage: 400 MB
```
