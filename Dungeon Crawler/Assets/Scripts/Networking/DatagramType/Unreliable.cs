namespace DungeonCrawler.Networking.Datagrams
{
    /// <summary>
    /// Sends a simple datagram, which will not
    /// be resent if droppped.
    /// </summary>
    public class Unreliable : Datagram
    {
        public string Data { get; set; }
        public Unreliable(string data) => Data = data;

        public static string CreateString(string data) => $"UNR::{data}";
    }
}