#include "demo/include/pcapplusplus/TcpReassembly.h"
#include "demo/include/pcapplusplus/PcapFileDevice.h"

#include "rust/cxx.h"

struct UserCookie;

void read_pcap(rust::Str fileName,
               UserCookie &userCookie,
               rust::Str bpfFilter);
