Simple to use command line program which allows you "Reverse Proxy" easily with built in windows tools.

# Uses
Useful if you're designing a website and want to easily be able to mimic a webserver locally.
For instance. Instead of using "127.0.0.1:8080" to access the webpages you can use "example.com" or if you have a sub domain "other.example.com"


# Commands

## List current proxy hosts.
```bash
localhosting.exe list
```

## Add new proxy host
```bash
localhosting.exe add <address> <host name>

# Example
localhosting.exe add 127.0.0.1:8080 example.com
localhosting.exe add 127.0.0.1:8080 proxy.example.com
```

## Remove proxy host
```bash
localhosting.exe remove <address/host name>

# Example
localhosting.exe remove 127.0.0.1:8080 # Removes anything using this ip:port.
localhosting.exe remove proxy.example.com # Removes only host.
```

## Test proxy host(s)
Starts up a webserver utilizing the previous ip:port combo provided when you added the host(s).

```bash
localhosting.exe test <address/host name>

# Example
localhosting.exe test 127.0.0.1:8080 # Test all hosts using this ip:port.
localhosting.exe test proxy.example.com # Test only this host.
```