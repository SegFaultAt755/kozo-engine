import kozo

magic = kozo.test_magic()
if magic != 0xC0FFEE:
    print("Invalid magic number")
else:
    print("The magic number is correct")
