# denpk2
Unpacks the NXPK files from games with NeoX 2 Engine.

## Information
The reverse engineering is done on the limited beta version of the Identity V game.
 > https://h55.gdl.netease.com/dwrg_nx2_gongyan_release_20240509.apk   
 > backup: https://drive.google.com/file/d/1GX5s3mg2-QzZQyL0lO7HsWagNKblWRiD/view

## Engine changes
The engine has updated it's embedded python from 2.7.3 to 3.11.6.   
And changes the rotor encryption in `redirect.nxs` to a `RSA` and `XOR` based custom encryption, 
the `redirect.nxs` is removed, and the script loading seems to be handled by the native code.   

The `marshal` format of the Python language stays untouched, 
so the `marshal` module can be used to load the unpacked code objects.
The opcodes has been shuffled as well, so the `dis` module can't be used to disassemble the code objects.
