using System.Net.Http.Json;

namespace BlazorData
{
     public class DataService(HttpClient http)
    {
        public async Task<List<Data>> GetEntries(int entryCount)
        {
            Data[] data = await http.GetFromJsonAsync<Data[]>($"data/{entryCount}") ?? [];

            return data.ToList();
        }
    }
}
