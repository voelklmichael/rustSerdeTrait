using System.Text.Json;
using System.Text.Json.Serialization;

namespace ExampleStandardWorking
{
    public interface ILabel
    {
        public string Label();
    }

    public class LabelConverter : JsonConverter<ILabel>
    {
        class ValueWithTypeName<T>(string TypeName, T Value)
        {
            public string TypeName { get; set; } = TypeName;
            public T Value { get; set; } = Value;
        }
        public override ILabel Read(
            ref Utf8JsonReader reader,
            Type typeToConvert,
            JsonSerializerOptions options)
        {
            throw new NotImplementedException();
        }

        public override void Write(
            Utf8JsonWriter writer,
            ILabel value,
            JsonSerializerOptions options)
        {
            switch (value)
            {
                case null:
                    JsonSerializer.Serialize(writer, (ILabel)null, options);
                    break;
                default:
                    {
                        var type = value.GetType();
                        var type_with_name = typeof(ValueWithTypeName<>).MakeGenericType(type);
                        var value_with_name = type_with_name.GetConstructor([typeof(string), type]).Invoke([type.FullName, value]);
                        JsonSerializer.Serialize(writer, value_with_name, type_with_name, options);
                        break;
                    }
            }
        }
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



    public class Program
    {
        public static void Main()
        {
            var a = new LabelA("Example A");
            var b = new LabelA("Example B");
            var serializedA = System.Text.Json.JsonSerializer.Serialize(a);
            Console.WriteLine(serializedA);

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
    }
}