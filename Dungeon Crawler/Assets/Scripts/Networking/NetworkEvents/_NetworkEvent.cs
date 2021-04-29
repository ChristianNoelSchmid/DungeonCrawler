
namespace DungeonCrawler.Networking.NetworkEvents
{
    /// <summary>
    /// The interface for all events that can
    /// be called from parsed udp datagram data.
    /// </summary>
    public interface NetworkEvent 
    {
        string CreateString();
    }
}