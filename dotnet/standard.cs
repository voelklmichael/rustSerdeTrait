namespace ExampleStandard
{
    public interface ILabel
    {
        public string Label();
    }

    public class LabelA(string Label) : ILabel
    {
        public string Label { get; set; } = Label;

        string ILabel.Label() => Label;
    }
    public class LabelB(string Label) : ILabel
    {
        public string Label { get; set; } = Label;
        string ILabel.Label() => Label;
    }

    public class Collection
    {
        public List<ILabel> Implementations { get; set; } = [];
    }
    public class CollectionUntyped
    {
        public List<Object> Implementations { get; set; } = [];
    }


    public class Program
    {
        public static void MainStandard()
        {
            var a = new LabelA("Example A");
            var b = new LabelA("Example B");

            {
                var list = new Collection() { Implementations = [a, b] };
                var serialized = System.Text.Json.JsonSerializer.Serialize(list);
                Console.WriteLine(serialized);
                try
                {
                    var deserialized = System.Text.Json.JsonSerializer.Deserialize<Collection>(serialized);
                    if (deserialized is not null)
                    {
                        Console.WriteLine("Labels: " + string.Join(", ", deserialized.Implementations.Select(x => x.Label())));
                    }
                    else
                    {
                        Console.WriteLine($"Failed to deserialize");
                    }
                }
                catch (Exception e) { Console.WriteLine($"Failed to deserialize: {e.Message}"); }
            }
            {
                var list = new CollectionUntyped() { Implementations = [a, b] };
                var serialized = System.Text.Json.JsonSerializer.Serialize(list);
                Console.WriteLine(serialized);
                try
                {
                    var deserialized = System.Text.Json.JsonSerializer.Deserialize<CollectionUntyped>(serialized);
                    if (deserialized is not null)
                    {
                        Console.WriteLine("Labels: " + string.Join(", ", deserialized.Implementations.Select(
                            x => ((x as ILabel) ?? throw new Exception("Type does not implemented interface")).Label())));
                    }
                    else
                    {
                        Console.WriteLine($"Failed to deserialize");
                    }
                }
                catch (Exception e) { Console.WriteLine($"Failed to deserialize: {e.Message}"); }

            }
        }
    }
}