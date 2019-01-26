import sys
for filename in sys.argv[1:]:
    with open(filename) as f:
        data = f.read()
    newdata = data.replace("/*!", "/*")
    if newdata != data:
        with open(filename, 'wb') as f:
            f.write(newdata)

