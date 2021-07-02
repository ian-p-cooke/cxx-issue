
#include "rust/cxx.h"

struct UserCookie;

void read_pcap(rust::Str fileName,
               UserCookie &userCookie,
               rust::Str bpfFilter);
