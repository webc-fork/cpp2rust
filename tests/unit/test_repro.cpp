#include <atomic>
#include <iostream>
#include <sstream>

void test_builtins() {
  if (false) {
    __builtin_unreachable();
  }
}

void test_format_escaping() {
  std::stringstream ss;
  ss << "{" << "key" << ":" << "value" << "}";
}

void test_atomics() {
  std::atomic_flag flag = ATOMIC_FLAG_INIT;
  flag.test_and_set();
  flag.clear();

  std::atomic<bool> b(true);
  b.store(false);
  bool val = b.load();

  std::atomic<unsigned long> ul(0);
  ul.fetch_add(1);
}

int main() {
  test_builtins();
  test_format_escaping();
  test_atomics();
  return 0;
}
