# jl: Jsonline Logger

An application for deserializing jsonline stdout and filtering output based on: 

- (1) log level (eg. info, warn, error, etc.)
- (2) fields (eg. appName, message, etc.)

Recommended usage:
```
kubectl logs -n <namespace> <pod name> | jl -l <level> -f <field1>,<field2>,<field3>,<etc> | less -r
```
