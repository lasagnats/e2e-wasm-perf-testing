namespace BlazorData
{
        public class Data
    {
        public int Id { get; set; } = default!;
        public string Label { get; set; } = default!;

        public Data(int id, string label)
        {
            Id = id;
            Label = label;
        }
    }
}
