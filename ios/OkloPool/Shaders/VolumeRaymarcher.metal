#include <metal_stdlib>
using namespace metal;

struct RaymarcherUniforms {
    packed_float3 camPos;
    float aspect;
    packed_float3 camRight;
    float opacity;
    packed_float3 camUp;
    int invertA;
    packed_float3 camFwd;
    int categorical;
};

static_assert(sizeof(RaymarcherUniforms) == 64, "RaymarcherUniforms size must be 64 bytes");
static_assert(alignof(RaymarcherUniforms) == 4, "RaymarcherUniforms alignment must be 4 bytes");
static_assert(__builtin_offsetof(RaymarcherUniforms, camPos) == 0, "camPos offset must be 0");
static_assert(__builtin_offsetof(RaymarcherUniforms, aspect) == 12, "aspect offset must be 12");
static_assert(__builtin_offsetof(RaymarcherUniforms, camRight) == 16, "camRight offset must be 16");
static_assert(__builtin_offsetof(RaymarcherUniforms, opacity) == 28, "opacity offset must be 28");
static_assert(__builtin_offsetof(RaymarcherUniforms, camUp) == 32, "camUp offset must be 32");
static_assert(__builtin_offsetof(RaymarcherUniforms, invertA) == 44, "invertA offset must be 44");
static_assert(__builtin_offsetof(RaymarcherUniforms, camFwd) == 48, "camFwd offset must be 48");
static_assert(__builtin_offsetof(RaymarcherUniforms, categorical) == 60, "categorical offset must be 60");


struct VertexIn {
    float2 position [[attribute(0)]];
};

struct VertexOut {
    float4 position [[position]];
    float2 uv;
};

vertex VertexOut volumeVertexShader(uint vertexID [[vertex_id]]) {
    VertexOut out;
    // Fullscreen quad from 4 vertices (triangle strip)
    float2 positions[4] = {
        float2(-1.0, -1.0),
        float2( 1.0, -1.0),
        float2(-1.0,  1.0),
        float2( 1.0,  1.0)
    };
    float2 pos = positions[vertexID];
    out.position = float4(pos, 0.0, 1.0);
    out.uv = pos * 0.5 + 0.5;
    return out;
}

fragment float4 volumeFragmentShader(
    VertexOut in [[stage_in]],
    constant RaymarcherUniforms &uni [[buffer(0)]],
    texture3d<float, access::sample> volTex [[texture(0)]],
    texture2d<float, access::sample> cmapTex [[texture(1)]],
    sampler volSampler [[sampler(0)]],
    sampler cmapSampler [[sampler(1)]])
{
    float2 p = in.uv * 2.0 - 1.0;
    float3 camPos = float3(uni.camPos);
    float3 camRight = float3(uni.camRight);
    float3 camUp = float3(uni.camUp);
    float3 camFwd = float3(uni.camFwd);

    float3 d = normalize(camFwd * 1.8 + camRight * p.x * uni.aspect + camUp * p.y);
    float3 inv = 1.0 / d;
    float3 t0 = -camPos * inv;
    float3 t1 = (float3(1.0) - camPos) * inv;
    float3 tn = min(t0, t1);
    float3 tx = max(t0, t1);

    float a = max(max(tn.x, tn.y), tn.z);
    float b = min(min(tx.x, tx.y), tx.z);

    if (a > b || b < 0.0) {
        return float4(0.0);
    }
    a = max(a, 0.0);

    const int STEPS = 192;
    float dt = 1.7320508 / float(STEPS);
    float4 acc = float4(0.0);

    for (int s = 0; s < STEPS; s++) {
        float t = a + (float(s) + 0.5) * dt;
        if (t > b) {
            break;
        }
        float3 q = camPos + d * t;
        float v = volTex.sample(volSampler, q).r;
        float4 c;
        if (uni.categorical == 1) {
            float coord = (v * 255.0 + 0.5) / 256.0;
            c = cmapTex.sample(cmapSampler, float2(coord, 0.5));
        } else {
            float4 rgbSample = cmapTex.sample(cmapSampler, float2(v, 0.5));
            float alphaVal = (uni.invertA == 1) ? (1.0 - v) : v;
            float smoothAlpha = smoothstep(0.25, 1.0, alphaVal);
            c = float4(rgbSample.rgb, smoothAlpha);
        }

        float al = clamp(c.a * uni.opacity * dt * 22.0, 0.0, 1.0);
        acc.rgb += (1.0 - acc.a) * al * c.rgb;
        acc.a += (1.0 - acc.a) * al;
        if (acc.a > 0.97) {
            break;
        }
    }

    return acc;
}
