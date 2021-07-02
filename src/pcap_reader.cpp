
#include "demo/include/pcap_reader.hpp"
#include "demo/src/main.rs.h"

#include "demo/include/pcapplusplus/TcpReassembly.h"
#include "demo/include/pcapplusplus/PcapFileDevice.h"

#define EXIT_WITH_ERROR(reason, ...) do { \
    printf("\nError: " reason "\n\n", ## __VA_ARGS__); \
    return; \
    } while(0)

void read_pcap(
        rust::Str fileName,
        UserCookie &userCookie,
        rust::Str bpfFilter) 
{
    /*
        typedef void(*       OnTcpMessageReady) (int8_t side, const TcpStreamData &tcpData, void *userCookie)
     */
    auto onMessageReady = [] (int8_t side, const pcpp::TcpStreamData &tcpData, void *userCookie) { 
      onMessageReadyCallback(side, tcpData, (UserCookie *)userCookie);
    };
    pcpp::TcpReassembly tcpReassembly(onMessageReady, &userCookie);

    // open input file (pcap or pcapng file)
    std::string fileName_s(fileName.data(), fileName.length());
    pcpp::IFileReaderDevice* reader = pcpp::IFileReaderDevice::getReader(fileName_s);

    // try to open the file device
    if (!reader->open()) {
        EXIT_WITH_ERROR("Cannot open pcap/pcapng file");
    }

    // set BPF filter if set by the user
    std::string bpfFilter_s(bpfFilter.data(), bpfFilter.length());
    if (!bpfFilter_s.empty())
    {
        if (!reader->setFilter(bpfFilter_s))
            EXIT_WITH_ERROR("Cannot set BPF filter to pcap file");
    }

    printf("Starting reading '%s'...\n", fileName_s.c_str());

    // run in a loop that reads one packet from the file in each iteration and feeds it to the TCP reassembly instance
    pcpp::RawPacket rawPacket;
    while (reader->getNextPacket(rawPacket))
    {
        tcpReassembly.reassemblePacket(&rawPacket);
    }

    // extract number of connections before closing all of them
    size_t numOfConnectionsProcessed = tcpReassembly.getConnectionInformation().size();

    // after all packets have been read - close the connections which are still opened
    tcpReassembly.closeAllConnections();

    // close the reader and free its memory
    reader->close();
    delete reader;

    printf("Done! processed %d connections\n", (int)numOfConnectionsProcessed);
}
