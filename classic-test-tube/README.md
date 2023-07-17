# classic-test-tube
a library for testing your smart contract against simulated classic chain logic. This library will allow smart contract developers to test contract against real chain logic without the need for spinning up a node. This will save tremendous amount of time by rapid testing.

To generate C header for use by test-tube `go tool cgo -exportheader artifacts/libclassictesttube.docrs.h main.go`
