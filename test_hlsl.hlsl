float4x4 worldMatrix;
float3 position : SV_Position;

struct VertexInput {
    float4 position : POSITION;
    float2 texCoord : TEXCOORD0;
};

struct PixelInput {
    float4 position : SV_Position;
    float2 texCoord : TEXCOORD0;
};

PixelInput VertexShader(VertexInput input) {
    PixelInput output;
    output.position = mul(input.position, worldMatrix);
    output.texCoord = input.texCoord;
    return output;
}
