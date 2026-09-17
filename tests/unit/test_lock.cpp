#include <csignal>
#include <atomic>

struct MetadataLock {
  MetadataLock() {}
  ~MetadataLock() {}
};

void test_lock() {
  MetadataLock lock;
}
