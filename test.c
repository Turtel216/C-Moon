int add(int x, int y) { return x + y; }

int main() {
    int x = 10;
    int y = 15;

    if (x > 100) {
        x = x + 1;
    }

    int z = add(x, y);

    return z;
}
