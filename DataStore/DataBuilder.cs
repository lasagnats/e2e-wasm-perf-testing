using DataStore.DataTypes;

namespace DataStore.DB
{
    public class DataBuilder
    {
        private readonly string[] adjectives = new string[]
        {
            "pretty",
            "large",
            "big",
            "small",
            "tall",
            "short",
            "long",
            "handsome",
            "plain",
            "quaint",
            "clean",
            "elegant",
            "easy",
            "angry",
            "crazy",
            "helpful",
            "mushy",
            "odd",
            "unsightly",
            "adorable",
            "important",
            "inexpensive",
            "cheap",
            "expensive",
            "fancy"
        };

        private readonly string[] colors = new string[]
        {
            "red",
            "yellow",
            "blue",
            "green",
            "pink",
            "brown",
            "purple",
            "brown",
            "white",
            "black",
            "orange"
        };

        private readonly string[] nouns = new string[]
        {
            "table",
            "chair",
            "house",
            "bbq",
            "desk",
            "car",
            "pony",
            "cookie",
            "sandwich",
            "burger",
            "pizza",
            "mouse",
            "keyboard"
        };

        public List<Data> BuildData(int count)
        {
            int rowId = 1;
            List<Data> result = new(count);
            for (int i = 0; i < count; i++)
            {
                Data data =
                    new()
                    {
                        Id = rowId++,
                        Label =
                            $"{adjectives[Random.Shared.Next(adjectives.Length)]} {colors[Random.Shared.Next(colors.Length)]} {nouns[Random.Shared.Next(nouns.Length)]}",
                    };

                result.Add(data);
            }

            return result;
        }
    }
}
