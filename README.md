# CLI command to output the details from your yubikey

A rustlang tool/eperiement to decode and display output from a YubiKey.

 - YubiKeys are hardware USB security keys made by [https://yubico.com]
 - ModHex format:  [https://en.wikipedia.org/wiki/YubiKey#ModHex]

Roughly based on a C language version I saw implemented on a youtube channel.

I hadn't heard of modhex before this so it was just a fun learning exercise,
specifically how to easily create a lookup table in rust using the array -> iter
-> clone -> collect flow. I am not sure if this would be considered good
practice or not but I find myself often wanting to concisely use lookup tables
like this rather than a long match or enum structure.

## Example output

```` shell
➜  yubiq git:(master) ✗ ./target/release/yubiq
touch your yubikey button now ...
ccccccrnddjfcitjjdddviijclkjhujvfnnekffkufuc
Yubi { code: "07d88222f7780a986e8f4bb39449e4e0", ssn: "000000cb2284" }

Binary Representation of the OTP is:
0000 0111 1101 1000
1000 0010 0010 0010
1111 0111 0111 1000
0000 1010 1001 1000
0110 1110 1000 1111
0100 1011 1011 0011
1001 0100 0100 1001
1110 0100 1110 0000
````

## Dependencies

This project does not have any external depdendencies.

```` shell
➜  yubiq git:(master) ✗ cargo tree    
yubiq v0.1.0 (/Users/mack/src/rust/yubiq)

````

