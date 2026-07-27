import os
def find_structs(directory):
    for root, dirs, files in os.walk(directory):
        for f in files:
            if f.endswith('.html'):
                print(os.path.join(root, f))
find_structs('target/doc/timestretch/engine')
