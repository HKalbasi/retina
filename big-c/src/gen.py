print('#include <stdio.h>')
print('void foo_0() { printf("salam"); }')
for i in range(10000):
    print(f"void foo_{i+1}()" + " { " + f"foo_{i}(); foo_{i}();" + " } ")
