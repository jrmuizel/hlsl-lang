// Test file demonstrating HLSL class support with methods

class MyClass {
    float value;
    float getValue() { return value; }
    void setValue(float v) { value = v; }
};

// Regular struct (no methods allowed)
struct MyStruct {
    float data;
    int count;
};

// Entry point to test different patterns
float4 main() : SV_Target {
    MyClass obj;
    
    // Test method calls on class
    obj.setValue(5.0);
    float result = obj.getValue();
    
    // Test struct usage (fields only)
    MyStruct s;
    s.data = result;
    s.count = 1;
    
    float4 color = float4(s.data, 0, 0, 1);
    
    return color;
}