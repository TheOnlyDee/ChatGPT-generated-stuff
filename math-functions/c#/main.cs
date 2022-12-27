public class MathLibrary
{
    public static double Sqrt(double x)
    {
        return Math.Sqrt(x);
    }

    public static double Power(double x, double y)
    {
        return Math.Pow(x, y);
    }

    public static double Log(double x)
    {
        return Math.Log(x);
    }

    public static double Cube(double x)
    {
        return x * x * x;
    }

    public static double CircleArea(double radius)
    {
        return Math.PI * radius * radius;
    }
}
