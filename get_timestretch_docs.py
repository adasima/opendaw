import os
import json

files = os.listdir('target/doc/timestretch')
print([f for f in files if f.endswith('.html')])

for d in ['engine']:
    if os.path.exists(f'target/doc/timestretch/{d}'):
        print(f"\n--- {d} ---")
        for f in os.listdir(f'target/doc/timestretch/{d}'):
             if f.endswith('.html'):
                print(f)
