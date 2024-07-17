namespace ExampleNewtonsoft
{
    public interface IInterface
    {
        public string Label();
    }

    public class ClassA(string Label) : IInterface
    {
        public string Label { get; set; } = Label;

        string IInterface.Label() => Label;
    }
    public class ClassB(string Label) : IInterface
    {
        public string Label { get; set; } = Label;
        string IInterface.Label() => Label;
    }

    public class Collection
    {
        public List<IInterface> Implementations { get; set; } = [];
    }


    public class Program
    {
        public static void MainNewtonSoft()
        {
            var a = new ClassA("Example A");
            var b = new ClassA("Example B");
            var list = new Collection() { Implementations = [a, b] };

            var serialized = Newtonsoft.Json.JsonConvert.SerializeObject(list);
            Console.WriteLine(serialized);
            try { var deserialized = Newtonsoft.Json.JsonConvert.DeserializeObject<Collection>(serialized); }
            catch (Exception e) { Console.WriteLine($"Failed to deserialize: {e.Message}"); }

            var options = new Newtonsoft.Json.JsonSerializerSettings()
            {
                TypeNameHandling = Newtonsoft.Json.TypeNameHandling.Auto
            };
            var serializedTyped = Newtonsoft.Json.JsonConvert.SerializeObject(list, options);
            Console.WriteLine(serializedTyped);
            var deserialized2 = Newtonsoft.Json.JsonConvert.DeserializeObject<Collection>(serializedTyped, options);
            if (deserialized2 is not null)
            {
                Console.WriteLine("Labels: " + string.Join(", ", deserialized2.Implementations.Select(x => x.Label())));
            }
        }
    }
}