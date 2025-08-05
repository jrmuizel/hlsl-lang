use lang_util::SmolStr;

use crate::{lexer, util::Unescaped};

use super::{
    keywords::KeywordAtom,
    type_names::{TypeNameAtom, TypeNameState},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Display)]
#[allow(non_camel_case_types)]
pub enum TypeName {
    /// "void"
    #[display("void")]
    VOID,
    /// "int"
    #[display("int")]
    INT,
    /// "bool"
    #[display("bool")]
    BOOL,
    /// "float"
    #[display("float")]
    FLOAT,
    /// "double"
    #[display("double")]
    DOUBLE,
    /// "uint"
    #[display("uint")]
    UINT,
    /// "float2" (HLSL vector type)
    #[display("float2")]
    VEC2,
    /// "float3" (HLSL vector type)
    #[display("float3")]
    VEC3,
    /// "float4" (HLSL vector type)
    #[display("float4")]
    VEC4,
    /// "int2" (HLSL vector type)
    #[display("int2")]
    IVEC2,
    /// "int3" (HLSL vector type)
    #[display("int3")]
    IVEC3,
    /// "int4" (HLSL vector type)
    #[display("int4")]
    IVEC4,
    /// "bool2" (HLSL vector type)
    #[display("bool2")]
    BVEC2,
    /// "bool3" (HLSL vector type)
    #[display("bool3")]
    BVEC3,
    /// "bool4" (HLSL vector type)
    #[display("bool4")]
    BVEC4,
    /// "uint2" (HLSL vector type)
    #[display("uint2")]
    UVEC2,
    /// "uint3" (HLSL vector type)
    #[display("uint3")]
    UVEC3,
    /// "uint4" (HLSL vector type)
    #[display("uint4")]
    UVEC4,
    /// "float2x2" (HLSL matrix type)
    #[display("float2x2")]
    MAT2,
    /// "float3x3" (HLSL matrix type)
    #[display("float3x3")]
    MAT3,
    /// "float4x4" (HLSL matrix type)
    #[display("float4x4")]
    MAT4,
    /// "float2x2" (HLSL matrix type)
    #[display("float2x2")]
    MAT2X2,
    /// "float2x3" (HLSL matrix type)
    #[display("float2x3")]
    MAT2X3,
    /// "float2x4" (HLSL matrix type)
    #[display("float2x4")]
    MAT2X4,
    /// "float3x2" (HLSL matrix type)
    #[display("float3x2")]
    MAT3X2,
    /// "float3x3" (HLSL matrix type)
    #[display("float3x3")]
    MAT3X3,
    /// "float3x4" (HLSL matrix type)
    #[display("float3x4")]
    MAT3X4,
    /// "float4x2" (HLSL matrix type)
    #[display("float4x2")]
    MAT4X2,
    /// "float4x3" (HLSL matrix type)
    #[display("float4x3")]
    MAT4X3,
    /// "float4x4" (HLSL matrix type)
    #[display("float4x4")]
    MAT4X4,
    /// "float1x1" through "float4x4" (HLSL matrix types)
    #[display("float1x1")]
    MAT1X1,
    /// "float1x2" (HLSL matrix type)
    #[display("float1x2")]
    MAT1X2,
    /// "float1x3" (HLSL matrix type)
    #[display("float1x3")]
    MAT1X3,
    /// "float1x4" (HLSL matrix type)
    #[display("float1x4")]
    MAT1X4,
    /// "float2x1" (HLSL matrix type)
    #[display("float2x1")]
    MAT2X1,
    /// "float3x1" (HLSL matrix type)
    #[display("float3x1")]
    MAT3X1,
    /// "float4x1" (HLSL matrix type)
    #[display("float4x1")]
    MAT4X1,
    /// "sampler" (HLSL sampler type)
    #[display("sampler")]
    SAMPLER,
    /// Reserved for future use
    RESERVED(TypeNameAtom),
    /// Generic type name
    OTHER(TypeNameAtom),
}

impl TypeName {
    fn gate(
        self,
        version_gate: bool,
        reserved_gate: bool,
        atom: TypeNameAtom,
        is_type_name: impl Fn(&TypeNameAtom) -> TypeNameState,
    ) -> Option<(Self, Option<TypeNameState>)> {
        // Check if the version gate allows this
        if version_gate {
            return Some((self, None));
        }

        // Check if an extension enabled this
        let result = is_type_name(&atom);
        if result.is_type_name() {
            return Some((self, Some(result)));
        }

        // Check if it's reserved
        if reserved_gate {
            return Some((Self::RESERVED(atom), None));
        }

        // Else it's unknown
        None
    }

    pub(crate) fn parse(
        name: &str,
        version: u16,
        target_vulkan: bool,
        is_type_name: impl Fn(&TypeNameAtom) -> TypeNameState,
    ) -> Option<(Self, Option<TypeNameState>)> {
        // TODO: Check type names rules for OpenGL ES

        use TypeName::*;

        let type_name_atom = TypeNameAtom::from(name);

        if type_name_atom == type_name!("void") {
            return VOID.gate(version >= 100, false, type_name_atom, is_type_name);
        } else if type_name_atom == type_name!("int") {
            return INT.gate(version >= 100, false, type_name_atom, is_type_name);
        } else if type_name_atom == type_name!("bool") {
            return BOOL.gate(version >= 100, false, type_name_atom, is_type_name);
        } else if type_name_atom == type_name!("float") {
            return FLOAT.gate(version >= 100, false, type_name_atom, is_type_name);
        } else if type_name_atom == type_name!("float2") {
            return VEC2.gate(version >= 100, false, type_name_atom, is_type_name);
        } else if type_name_atom == type_name!("float3") {
            return VEC3.gate(version >= 100, false, type_name_atom, is_type_name);
        } else if type_name_atom == type_name!("float4") {
            return VEC4.gate(version >= 100, false, type_name_atom, is_type_name);
        } else if type_name_atom == type_name!("int2") {
            return IVEC2.gate(version >= 100, false, type_name_atom, is_type_name);
        } else if type_name_atom == type_name!("int3") {
            return IVEC3.gate(version >= 100, false, type_name_atom, is_type_name);
        } else if type_name_atom == type_name!("int4") {
            return IVEC4.gate(version >= 100, false, type_name_atom, is_type_name);
        } else if type_name_atom == type_name!("bool2") {
            return BVEC2.gate(version >= 100, false, type_name_atom, is_type_name);
        } else if type_name_atom == type_name!("bool3") {
            return BVEC3.gate(version >= 100, false, type_name_atom, is_type_name);
        } else if type_name_atom == type_name!("bool4") {
            return BVEC4.gate(version >= 100, false, type_name_atom, is_type_name);
        } else if type_name_atom == type_name!("float2x2") {
            return MAT2X2.gate(version >= 100, false, type_name_atom, is_type_name);
        } else if type_name_atom == type_name!("float3x3") {
            return MAT3X3.gate(version >= 100, false, type_name_atom, is_type_name);
        } else if type_name_atom == type_name!("float4x4") {
            return MAT4X4.gate(version >= 100, false, type_name_atom, is_type_name);
        } else if type_name_atom == type_name!("sampler") {
            return SAMPLER.gate(version >= 100, false, type_name_atom, is_type_name);
        } else if type_name_atom == type_name!("uint") {
            return UINT.gate(version >= 100, false, type_name_atom, is_type_name);
        } else if type_name_atom == type_name!("uint2") {
            return UVEC2.gate(version >= 100, false, type_name_atom, is_type_name);
        } else if type_name_atom == type_name!("uint3") {
            return UVEC3.gate(version >= 100, false, type_name_atom, is_type_name);
        } else if type_name_atom == type_name!("uint4") {
            return UVEC4.gate(version >= 100, false, type_name_atom, is_type_name);
        } else if type_name_atom == type_name!("double") {
            return DOUBLE.gate(version >= 100, false, type_name_atom, is_type_name);
        }

        // Additional HLSL matrix types
        if type_name_atom == type_name!("float2x1") {
            return MAT2X1.gate(version >= 100, false, type_name_atom, is_type_name);
        } else if type_name_atom == type_name!("float2x3") {
            return MAT2X3.gate(version >= 100, false, type_name_atom, is_type_name);
        } else if type_name_atom == type_name!("float2x4") {
            return MAT2X4.gate(version >= 100, false, type_name_atom, is_type_name);
        } else if type_name_atom == type_name!("float3x2") {
            return MAT3X2.gate(version >= 100, false, type_name_atom, is_type_name);
        } else if type_name_atom == type_name!("float3x4") {
            return MAT3X4.gate(version >= 100, false, type_name_atom, is_type_name);
        } else if type_name_atom == type_name!("float4x2") {
            return MAT4X2.gate(version >= 100, false, type_name_atom, is_type_name);
        } else if type_name_atom == type_name!("float4x3") {
            return MAT4X3.gate(version >= 100, false, type_name_atom, is_type_name);
        } else if type_name_atom == type_name!("float1x1") {
            return MAT1X1.gate(version >= 100, false, type_name_atom, is_type_name);
        } else if type_name_atom == type_name!("float1x2") {
            return MAT1X2.gate(version >= 100, false, type_name_atom, is_type_name);
        } else if type_name_atom == type_name!("float1x3") {
            return MAT1X3.gate(version >= 100, false, type_name_atom, is_type_name);
        } else if type_name_atom == type_name!("float1x4") {
            return MAT1X4.gate(version >= 100, false, type_name_atom, is_type_name);
        } else if type_name_atom == type_name!("float3x1") {
            return MAT3X1.gate(version >= 100, false, type_name_atom, is_type_name);
        } else if type_name_atom == type_name!("float4x1") {
            return MAT4X1.gate(version >= 100, false, type_name_atom, is_type_name);
        }

        let result = is_type_name(&type_name_atom);
        if result.is_type_name() {
            return Some((OTHER(type_name_atom), Some(result)));
        }

        None
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, lang_util::Token)]
#[allow(non_camel_case_types)]
pub enum Token {
    /// Identifier
    #[lang_util(parser = "IDENT", kind = "identifier")]
    IDENT(SmolStr),
    /// Type name
    #[lang_util(parser = "TYPE_NAME", kind = "type name")]
    TYPE_NAME(TypeName),
    /// Float constant
    #[lang_util(parser = "FLOAT_CONST", kind = "float constant", kind = "literal")]
    FLOAT_CONST(f32),
    /// Int constant
    #[lang_util(parser = "INT_CONST", kind = "int constant", kind = "literal")]
    INT_CONST(i32),
    /// Unsigned int constant
    #[lang_util(parser = "UINT_CONST", kind = "uint constant", kind = "literal")]
    UINT_CONST(u32),
    /// Bool constant
    #[lang_util(parser = "BOOL_CONST", kind = "bool constant", kind = "literal")]
    BOOL_CONST(bool),
    /// Double constant
    #[lang_util(parser = "DOUBLE_CONST", kind = "double constant", kind = "literal")]
    DOUBLE_CONST(f64),
    // Multi-char tokens
    /// <<
    #[lang_util(token = "<<", kind = "binary operator", kind = "operator")]
    LEFT_OP,
    /// >>
    #[lang_util(token = ">>", kind = "binary operator", kind = "operator")]
    RIGHT_OP,
    /// ++
    #[lang_util(token = "++", kind = "unary operator", kind = "operator")]
    INC_OP,
    /// --
    #[lang_util(token = "--", kind = "unary operator", kind = "operator")]
    DEC_OP,
    /// <=
    #[lang_util(token = "<=", kind = "binary operator", kind = "operator")]
    LE_OP,
    /// >=
    #[lang_util(token = ">=", kind = "binary operator", kind = "operator")]
    GE_OP,
    /// ==
    #[lang_util(token = "==", kind = "binary operator", kind = "operator")]
    EQ_OP,
    /// !=
    #[lang_util(token = "!=", kind = "binary operator", kind = "operator")]
    NE_OP,
    /// &&
    #[lang_util(token = "&&", kind = "binary operator", kind = "operator")]
    AND_OP,
    /// ||
    #[lang_util(token = "||", kind = "binary operator", kind = "operator")]
    OR_OP,
    /// ^^
    #[lang_util(token = "^^", kind = "binary operator", kind = "operator")]
    XOR_OP,
    /// *=
    #[lang_util(token = "*=", kind = "binary operator", kind = "operator")]
    MUL_ASSIGN,
    /// /=
    #[lang_util(token = "/=", kind = "binary operator", kind = "operator")]
    DIV_ASSIGN,
    /// +=
    #[lang_util(token = "+=", kind = "binary operator", kind = "operator")]
    ADD_ASSIGN,
    /// %=
    #[lang_util(token = "%=", kind = "binary operator", kind = "operator")]
    MOD_ASSIGN,
    /// <<=
    #[lang_util(token = "<<=", kind = "binary operator", kind = "operator")]
    LEFT_ASSIGN,
    /// >>=
    #[lang_util(token = ">>=", kind = "binary operator", kind = "operator")]
    RIGHT_ASSIGN,
    /// &=
    #[lang_util(token = "&=", kind = "binary operator", kind = "operator")]
    AND_ASSIGN,
    /// ^=
    #[lang_util(token = "^=", kind = "binary operator", kind = "operator")]
    XOR_ASSIGN,
    /// |=
    #[lang_util(token = "|=", kind = "binary operator", kind = "operator")]
    OR_ASSIGN,
    /// -=
    #[lang_util(token = "-=", kind = "binary operator", kind = "operator")]
    SUB_ASSIGN,
    // Single-char tokens
    /// (
    #[lang_util(token = "(")]
    LPAREN,
    /// )
    #[lang_util(token = ")")]
    RPAREN,
    /// [
    #[lang_util(token = "[")]
    LBRACKET,
    /// ]
    #[lang_util(token = "]")]
    RBRACKET,
    /// {
    #[lang_util(token = "{")]
    LBRACE,
    /// }
    #[lang_util(token = "}")]
    RBRACE,
    /// .
    #[lang_util(token = ".", kind = "binary operator", kind = "operator")]
    PERIOD,
    /// ,
    #[lang_util(token = ",", kind = "operator")]
    COMMA,
    /// :
    #[lang_util(token = ":", kind = "operator")]
    COLON,
    /// =
    #[lang_util(token = "=", kind = "binary operator", kind = "operator")]
    EQUAL,
    /// ;
    #[lang_util(token = ";")]
    SEMICOLON,
    /// !
    #[lang_util(token = "!", kind = "unary operator", kind = "operator")]
    BANG,
    /// -
    #[lang_util(
        token = "-",
        kind = "binary operator",
        kind = "unary operator",
        kind = "operator"
    )]
    DASH,
    /// ~
    #[lang_util(token = "~", kind = "unary operator", kind = "operator")]
    TILDE,
    /// +
    #[lang_util(
        token = "+",
        kind = "binary operator",
        kind = "unary operator",
        kind = "operator"
    )]
    PLUS,
    /// *
    #[lang_util(token = "*", kind = "binary operator", kind = "operator")]
    ASTERISK,
    /// /
    #[lang_util(token = "/", kind = "binary operator", kind = "operator")]
    SLASH,
    /// %
    #[lang_util(token = "%", kind = "binary operator", kind = "operator")]
    PERCENT,
    /// <
    #[lang_util(token = "<", kind = "binary operator", kind = "operator")]
    LANGLE,
    /// >
    #[lang_util(token = ">", kind = "binary operator", kind = "operator")]
    RANGLE,
    /// |
    #[lang_util(token = "|", kind = "binary operator", kind = "operator")]
    BAR,
    /// ^
    #[lang_util(token = "^", kind = "binary operator", kind = "operator")]
    CARET,
    /// &
    #[lang_util(token = "&", kind = "binary operator", kind = "operator")]
    AMPERSAND,
    /// ?
    #[lang_util(token = "?", kind = "operator")]
    QUESTION,
    /// #
    #[lang_util(token = "#")]
    HASH,
    // Keywords
    /// "const"
    #[lang_util(token = "const", kind = "storage qualifier", kind = "type qualifier")]
    CONST,
    /// "uniform"
    #[lang_util(token = "uniform", kind = "storage qualifier", kind = "type qualifier")]
    UNIFORM,
    /// "buffer"
    #[lang_util(token = "buffer", kind = "storage qualifier", kind = "type qualifier")]
    BUFFER,
    /// "shared"
    #[lang_util(token = "shared", kind = "storage qualifier", kind = "type qualifier")]
    SHARED,
    /// "attribute"
    #[lang_util(
        token = "attribute",
        kind = "storage qualifier",
        kind = "type qualifier"
    )]
    ATTRIBUTE,
    /// "varying"
    #[lang_util(token = "varying", kind = "storage qualifier", kind = "type qualifier")]
    VARYING,
    /// "coherent"
    #[lang_util(
        token = "coherent",
        kind = "storage qualifier",
        kind = "type qualifier"
    )]
    COHERENT,
    /// "volatile"
    #[lang_util(
        token = "volatile",
        kind = "storage qualifier",
        kind = "type qualifier"
    )]
    VOLATILE,
    /// "restrict"
    #[lang_util(
        token = "restrict",
        kind = "storage qualifier",
        kind = "type qualifier"
    )]
    RESTRICT,
    /// "readonly"
    #[lang_util(
        token = "readonly",
        kind = "storage qualifier",
        kind = "type qualifier"
    )]
    READONLY,
    /// "writeonly"
    #[lang_util(
        token = "writeonly",
        kind = "storage qualifier",
        kind = "type qualifier"
    )]
    WRITEONLY,
    /// "layout"
    #[lang_util(token = "layout", kind = "layout qualifier", kind = "type qualifier")]
    LAYOUT,
    /// "centroid"
    #[lang_util(
        token = "centroid",
        kind = "storage qualifier",
        kind = "type qualifier"
    )]
    CENTROID,
    /// "flat"
    #[lang_util(
        token = "flat",
        kind = "interpolation qualifier",
        kind = "type qualifier"
    )]
    FLAT,
    /// "smooth"
    #[lang_util(
        token = "smooth",
        kind = "interpolation qualifier",
        kind = "type qualifier"
    )]
    SMOOTH,
    /// "noperspective"
    #[lang_util(
        token = "noperspective",
        kind = "interpolation qualifier",
        kind = "type qualifier"
    )]
    NOPERSPECTIVE,
    /// "patch"
    #[lang_util(token = "patch", kind = "storage qualifier", kind = "type qualifier")]
    PATCH,
    /// "sample"
    #[lang_util(token = "sample", kind = "storage qualifier", kind = "type qualifier")]
    SAMPLE,
    /// "invariant"
    #[lang_util(token = "invariant", kind = "type qualifier")]
    INVARIANT,
    /// "precise"
    #[lang_util(token = "precise", kind = "type qualifier")]
    PRECISE,
    /// "break"
    #[lang_util(token = "break", kind = "keyword")]
    BREAK,
    /// "continue"
    #[lang_util(token = "continue", kind = "keyword")]
    CONTINUE,
    /// "do"
    #[lang_util(token = "do", kind = "keyword")]
    DO,
    /// "for"
    #[lang_util(token = "for", kind = "keyword")]
    FOR,
    /// "while"
    #[lang_util(token = "while", kind = "keyword")]
    WHILE,
    /// "switch"
    #[lang_util(token = "switch", kind = "keyword")]
    SWITCH,
    /// "case"
    #[lang_util(token = "case", kind = "keyword")]
    CASE,
    /// "default"
    #[lang_util(token = "default", kind = "keyword")]
    DEFAULT,
    /// "if"
    #[lang_util(token = "if", kind = "keyword")]
    IF,
    /// "else"
    #[lang_util(token = "else", kind = "keyword")]
    ELSE,
    /// "subroutine"
    #[lang_util(
        token = "subroutine",
        kind = "storage qualifier",
        kind = "type qualifier"
    )]
    SUBROUTINE,
    /// "in"
    #[lang_util(token = "in", kind = "storage qualifier", kind = "type qualifier")]
    IN,
    /// "out"
    #[lang_util(token = "out", kind = "storage qualifier", kind = "type qualifier")]
    OUT,
    /// "inout"
    #[lang_util(token = "inout", kind = "storage qualifier", kind = "type qualifier")]
    INOUT,
    /// "discard"
    #[lang_util(token = "discard", kind = "keyword")]
    DISCARD,
    /// "return"
    #[lang_util(token = "return", kind = "keyword")]
    RETURN,
    /// "lowp"
    #[lang_util(token = "lowp", kind = "precision qualifier", kind = "type qualifier")]
    LOWP,
    /// "mediump"
    #[lang_util(
        token = "mediump",
        kind = "precision qualifier",
        kind = "type qualifier"
    )]
    MEDIUMP,
    /// "highp"
    #[lang_util(token = "highp", kind = "precision qualifier", kind = "type qualifier")]
    HIGHP,
    /// "precision"
    #[lang_util(token = "precision")]
    PRECISION,
    /// "struct"
    #[lang_util(token = "struct", kind = "struct", kind = "keyword")]
    STRUCT,
    /// "cbuffer"
    #[lang_util(token = "cbuffer", kind = "keyword")]
    CBUFFER,
    /// "SamplerState"
    #[lang_util(token = "SamplerState", kind = "type name")]
    SAMPLER_STATE,
    /// "SamplerComparisonState"
    #[lang_util(token = "SamplerComparisonState", kind = "type name")]
    SAMPLER_COMPARISON_STATE,
    // Reserved for future use
    /// "common"
    #[lang_util(token = "common", kind = "reserved keyword")]
    COMMON,
    /// "partition"
    #[lang_util(token = "partition", kind = "reserved keyword")]
    PARTITION,
    /// "active"
    #[lang_util(token = "active", kind = "reserved keyword")]
    ACTIVE,
    /// "asm"
    #[lang_util(token = "asm", kind = "reserved keyword")]
    ASM,
    /// "class"
    #[lang_util(token = "class", kind = "reserved keyword")]
    CLASS,
    /// "union"
    #[lang_util(token = "union", kind = "reserved keyword")]
    UNION,
    /// "enum"
    #[lang_util(token = "enum", kind = "reserved keyword")]
    ENUM,
    /// "typedef"
    #[lang_util(token = "typedef", kind = "reserved keyword")]
    TYPEDEF,
    /// "template"
    #[lang_util(token = "template", kind = "reserved keyword")]
    TEMPLATE,
    /// "this"
    #[lang_util(token = "this", kind = "reserved keyword")]
    THIS,
    /// "resource"
    #[lang_util(token = "resource", kind = "reserved keyword")]
    RESOURCE,
    /// "goto"
    #[lang_util(token = "goto", kind = "reserved keyword")]
    GOTO,
    /// "inline"
    #[lang_util(token = "inline", kind = "reserved keyword")]
    INLINE,
    /// "noinline"
    #[lang_util(token = "noinline", kind = "reserved keyword")]
    NOINLINE,
    /// "public"
    #[lang_util(token = "public", kind = "reserved keyword")]
    PUBLIC,
    /// "static"
    #[lang_util(token = "static", kind = "reserved keyword")]
    STATIC,
    /// "extern"
    #[lang_util(token = "extern", kind = "reserved keyword")]
    EXTERN,
    /// "external"
    #[lang_util(token = "external", kind = "reserved keyword")]
    EXTERNAL,
    /// "interface"
    #[lang_util(token = "interface", kind = "reserved keyword")]
    INTERFACE,
    /// "long"
    #[lang_util(token = "long", kind = "reserved keyword")]
    LONG,
    /// "short"
    #[lang_util(token = "short", kind = "reserved keyword")]
    SHORT,
    /// "half"
    #[lang_util(token = "half", kind = "reserved keyword")]
    HALF,
    /// "fixed"
    #[lang_util(token = "fixed", kind = "reserved keyword")]
    FIXED,
    /// "unsigned"
    #[lang_util(token = "unsigned", kind = "reserved keyword")]
    UNSIGNED,
    /// "superp"
    #[lang_util(token = "superp", kind = "reserved keyword")]
    SUPERP,
    /// "input"
    #[lang_util(token = "input", kind = "reserved keyword")]
    INPUT,
    /// "output"
    #[lang_util(token = "output", kind = "reserved keyword")]
    OUTPUT,
    /// "filter"
    #[lang_util(token = "filter", kind = "reserved keyword")]
    FILTER,
    /// "sizeof"
    #[lang_util(token = "sizeof", kind = "reserved keyword")]
    SIZEOF,
    /// "cast"
    #[lang_util(token = "cast", kind = "reserved keyword")]
    CAST,
    /// "namespace"
    #[lang_util(token = "namespace", kind = "reserved keyword")]
    NAMESPACE,
    /// "using"
    #[lang_util(token = "using", kind = "reserved keyword")]
    USING,
    // Other
    /// Whitespaace
    #[lang_util(display = "<whitespace>", parser(display), kind = "trivia")]
    WS,
    /// Comment (single-line or multi-line)
    #[lang_util(display = "<comment>", parser(display), kind = "trivia")]
    COMMENT,
    /// Marker for invalid tokens
    #[lang_util(display = "<invalid token>", parser(display), kind = "error")]
    ERROR(ErrorKind),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Display)]
pub enum ErrorKind {
    #[display("invalid token")]
    InvalidToken,
    #[display("invalid int literal")]
    InvalidIntLiteral,
    #[display("invalid uint literal")]
    InvalidUIntLiteral,
    #[display("invalid float literal")]
    InvalidFloatLiteral,
    #[display("invalid double literal")]
    InvalidDoubleLiteral,
}

impl Token {
    pub fn from_token(
        token: lexer::TextToken,
        source: &str,
        version: u16,
        target_vulkan: bool,
        is_type_name: impl Fn(&TypeNameAtom) -> TypeNameState,
    ) -> (Token, Option<TypeNameState>) {
        use Token::*;
        let kind = &*token;

        match kind {
            lexer::Token::IDENT_KW | lexer::Token::DIGITS => {}
            lexer::Token::PERIOD => {
                return (PERIOD, None);
            }
            lexer::Token::PLUS => {
                return (PLUS, None);
            }
            lexer::Token::DASH => {
                return (DASH, None);
            }
            lexer::Token::SLASH => {
                return (SLASH, None);
            }
            lexer::Token::ASTERISK => {
                return (ASTERISK, None);
            }
            lexer::Token::PERCENT => {
                return (PERCENT, None);
            }
            lexer::Token::LANGLE => {
                return (LANGLE, None);
            }
            lexer::Token::RANGLE => {
                return (RANGLE, None);
            }
            lexer::Token::LBRACKET => {
                return (LBRACKET, None);
            }
            lexer::Token::RBRACKET => {
                return (RBRACKET, None);
            }
            lexer::Token::LPAREN => {
                return (LPAREN, None);
            }
            lexer::Token::RPAREN => {
                return (RPAREN, None);
            }
            lexer::Token::LBRACE => {
                return (LBRACE, None);
            }
            lexer::Token::RBRACE => {
                return (RBRACE, None);
            }
            lexer::Token::CARET => {
                return (CARET, None);
            }
            lexer::Token::BAR => {
                return (BAR, None);
            }
            lexer::Token::AMPERSAND => {
                return (AMPERSAND, None);
            }
            lexer::Token::TILDE => {
                return (TILDE, None);
            }
            lexer::Token::EQUAL => {
                return (EQUAL, None);
            }
            lexer::Token::BANG => {
                return (BANG, None);
            }
            lexer::Token::COLON => {
                return (COLON, None);
            }
            lexer::Token::SEMICOLON => {
                return (SEMICOLON, None);
            }
            lexer::Token::COMMA => {
                return (COMMA, None);
            }
            lexer::Token::QUESTION => {
                return (QUESTION, None);
            }
            lexer::Token::HASH => {
                return (HASH, None);
            }
            lexer::Token::COMMENT => {
                return (COMMENT, None);
            }
            lexer::Token::LEFT_OP => {
                return (LEFT_OP, None);
            }
            lexer::Token::RIGHT_OP => {
                return (RIGHT_OP, None);
            }
            lexer::Token::INC_OP => {
                return (INC_OP, None);
            }
            lexer::Token::DEC_OP => {
                return (DEC_OP, None);
            }
            lexer::Token::LE_OP => {
                return (LE_OP, None);
            }
            lexer::Token::GE_OP => {
                return (GE_OP, None);
            }
            lexer::Token::EQ_OP => {
                return (EQ_OP, None);
            }
            lexer::Token::NE_OP => {
                return (NE_OP, None);
            }
            lexer::Token::AND_OP => {
                return (AND_OP, None);
            }
            lexer::Token::OR_OP => {
                return (OR_OP, None);
            }
            lexer::Token::XOR_OP => {
                return (XOR_OP, None);
            }
            lexer::Token::MUL_ASSIGN => {
                return (MUL_ASSIGN, None);
            }
            lexer::Token::DIV_ASSIGN => {
                return (DIV_ASSIGN, None);
            }
            lexer::Token::ADD_ASSIGN => {
                return (ADD_ASSIGN, None);
            }
            lexer::Token::MOD_ASSIGN => {
                return (MOD_ASSIGN, None);
            }
            lexer::Token::LEFT_ASSIGN => {
                return (LEFT_ASSIGN, None);
            }
            lexer::Token::RIGHT_ASSIGN => {
                return (RIGHT_ASSIGN, None);
            }
            lexer::Token::AND_ASSIGN => {
                return (AND_ASSIGN, None);
            }
            lexer::Token::XOR_ASSIGN => {
                return (XOR_ASSIGN, None);
            }
            lexer::Token::OR_ASSIGN => {
                return (OR_ASSIGN, None);
            }
            lexer::Token::SUB_ASSIGN => {
                return (SUB_ASSIGN, None);
            }
            lexer::Token::DEFINED
            | lexer::Token::LINECONT
            | lexer::Token::NEWLINE
            | lexer::Token::WS => {
                return (WS, None);
            }
            lexer::Token::QUOTE_STRING
            | lexer::Token::ANGLE_STRING
            | lexer::Token::BACKSLASH
            | lexer::Token::ERROR
            | lexer::Token::PP_CONCAT => {
                return (ERROR(ErrorKind::InvalidToken), None);
            }
        }

        // Either IDENT_KW or DIGITS, we need to examine the text to know more
        let text = Unescaped::new(token.raw(source)).to_string();
        if *kind == lexer::Token::IDENT_KW {
            // Is this a keyword?
            let keyword_atom = KeywordAtom::from(text.as_ref());

            if let Some(keyword) = Token::parse_kw(&keyword_atom) {
                return (keyword, None);
            }

            // Else it might be a built-in type name
            if let Some((type_name, state)) =
                TypeName::parse(text.as_ref(), version, target_vulkan, is_type_name)
            {
                return (TYPE_NAME(type_name), state);
            }

            // Nothing matched, it's actually an ident
            (IDENT(text.into()), None)
        } else if *kind == lexer::Token::DIGITS {
            (Token::parse_digits(&text), None)
        } else {
            unreachable!()
        }
    }

    pub(crate) fn parse_kw(keyword_atom: &KeywordAtom) -> Option<Self> {
        use Token::*;

        // Keywords
        // TODO: Only return keywords according to versions and extensions
        if *keyword_atom == keyword!("const") {
            Some(CONST)
        } else if *keyword_atom == keyword!("uniform") {
            Some(UNIFORM)
        } else if *keyword_atom == keyword!("buffer") {
            Some(BUFFER)
        } else if *keyword_atom == keyword!("shared") {
            Some(SHARED)
        } else if *keyword_atom == keyword!("attribute") {
            Some(ATTRIBUTE)
        } else if *keyword_atom == keyword!("varying") {
            Some(VARYING)
        } else if *keyword_atom == keyword!("coherent") {
            Some(COHERENT)
        } else if *keyword_atom == keyword!("volatile") {
            Some(VOLATILE)
        } else if *keyword_atom == keyword!("restrict") {
            Some(RESTRICT)
        } else if *keyword_atom == keyword!("readonly") {
            Some(READONLY)
        } else if *keyword_atom == keyword!("writeonly") {
            Some(WRITEONLY)
        } else if *keyword_atom == keyword!("layout") {
            Some(LAYOUT)
        } else if *keyword_atom == keyword!("centroid") {
            Some(CENTROID)
        } else if *keyword_atom == keyword!("flat") {
            Some(FLAT)
        } else if *keyword_atom == keyword!("smooth") {
            Some(SMOOTH)
        } else if *keyword_atom == keyword!("noperspective") {
            Some(NOPERSPECTIVE)
        } else if *keyword_atom == keyword!("patch") {
            Some(PATCH)
        } else if *keyword_atom == keyword!("sample") {
            Some(SAMPLE)
        } else if *keyword_atom == keyword!("invariant") {
            Some(INVARIANT)
        } else if *keyword_atom == keyword!("precise") {
            Some(PRECISE)
        } else if *keyword_atom == keyword!("break") {
            Some(BREAK)
        } else if *keyword_atom == keyword!("continue") {
            Some(CONTINUE)
        } else if *keyword_atom == keyword!("do") {
            Some(DO)
        } else if *keyword_atom == keyword!("for") {
            Some(FOR)
        } else if *keyword_atom == keyword!("while") {
            Some(WHILE)
        } else if *keyword_atom == keyword!("switch") {
            Some(SWITCH)
        } else if *keyword_atom == keyword!("case") {
            Some(CASE)
        } else if *keyword_atom == keyword!("default") {
            Some(DEFAULT)
        } else if *keyword_atom == keyword!("if") {
            Some(IF)
        } else if *keyword_atom == keyword!("else") {
            Some(ELSE)
        } else if *keyword_atom == keyword!("subroutine") {
            Some(SUBROUTINE)
        } else if *keyword_atom == keyword!("in") {
            Some(IN)
        } else if *keyword_atom == keyword!("out") {
            Some(OUT)
        } else if *keyword_atom == keyword!("inout") {
            Some(INOUT)
        } else if *keyword_atom == keyword!("true") {
            Some(BOOL_CONST(true))
        } else if *keyword_atom == keyword!("false") {
            Some(BOOL_CONST(false))
        } else if *keyword_atom == keyword!("discard") {
            Some(DISCARD)
        } else if *keyword_atom == keyword!("return") {
            Some(RETURN)
        } else if *keyword_atom == keyword!("lowp") {
            Some(LOWP)
        } else if *keyword_atom == keyword!("mediump") {
            Some(MEDIUMP)
        } else if *keyword_atom == keyword!("highp") {
            Some(HIGHP)
        } else if *keyword_atom == keyword!("precision") {
            Some(PRECISION)
        } else if *keyword_atom == keyword!("struct") {
            Some(STRUCT)
        } else if *keyword_atom == keyword!("cbuffer") {
            Some(CBUFFER)
        } else if *keyword_atom == keyword!("SamplerState") {
            Some(SAMPLER_STATE)
        } else if *keyword_atom == keyword!("SamplerComparisonState") {
            Some(SAMPLER_COMPARISON_STATE)
        }
        // Reserved for future use
        else if *keyword_atom == keyword!("common") {
            Some(COMMON)
        } else if *keyword_atom == keyword!("partition") {
            Some(PARTITION)
        } else if *keyword_atom == keyword!("active") {
            Some(ACTIVE)
        } else if *keyword_atom == keyword!("asm") {
            Some(ASM)
        } else if *keyword_atom == keyword!("class") {
            Some(CLASS)
        } else if *keyword_atom == keyword!("union") {
            Some(UNION)
        } else if *keyword_atom == keyword!("enum") {
            Some(ENUM)
        } else if *keyword_atom == keyword!("typedef") {
            Some(TYPEDEF)
        } else if *keyword_atom == keyword!("template") {
            Some(TEMPLATE)
        } else if *keyword_atom == keyword!("this") {
            Some(THIS)
        } else if *keyword_atom == keyword!("resource") {
            Some(RESOURCE)
        } else if *keyword_atom == keyword!("goto") {
            Some(GOTO)
        } else if *keyword_atom == keyword!("inline") {
            Some(INLINE)
        } else if *keyword_atom == keyword!("noinline") {
            Some(NOINLINE)
        } else if *keyword_atom == keyword!("public") {
            Some(PUBLIC)
        } else if *keyword_atom == keyword!("static") {
            Some(STATIC)
        } else if *keyword_atom == keyword!("extern") {
            Some(EXTERN)
        } else if *keyword_atom == keyword!("external") {
            Some(EXTERNAL)
        } else if *keyword_atom == keyword!("interface") {
            Some(INTERFACE)
        } else if *keyword_atom == keyword!("long") {
            Some(LONG)
        } else if *keyword_atom == keyword!("short") {
            Some(SHORT)
        } else if *keyword_atom == keyword!("half") {
            Some(HALF)
        } else if *keyword_atom == keyword!("fixed") {
            Some(FIXED)
        } else if *keyword_atom == keyword!("unsigned") {
            Some(UNSIGNED)
        } else if *keyword_atom == keyword!("superp") {
            Some(SUPERP)
        } else if *keyword_atom == keyword!("input") {
            Some(INPUT)
        } else if *keyword_atom == keyword!("output") {
            Some(OUTPUT)
        } else if *keyword_atom == keyword!("filter") {
            Some(FILTER)
        } else if *keyword_atom == keyword!("sizeof") {
            Some(SIZEOF)
        } else if *keyword_atom == keyword!("cast") {
            Some(CAST)
        } else if *keyword_atom == keyword!("namespace") {
            Some(NAMESPACE)
        } else if *keyword_atom == keyword!("using") {
            Some(USING)
        } else {
            None
        }
    }

    fn strip_suffix(text: &str) -> (bool, &str) {
        if let Some(stripped) = text.strip_suffix(['u', 'U']) {
            (true, stripped)
        } else {
            (false, text)
        }
    }

    fn parse_int(text: &str, radix: u32) -> Result<Self, ErrorKind> {
        use Token::*;

        let (unsigned, text) = Self::strip_suffix(text);

        // Hexadecimal constant
        if unsigned {
            if radix == 8 && text.is_empty() {
                Ok(UINT_CONST(0))
            } else {
                u32::from_str_radix(text, radix)
                    .map(UINT_CONST)
                    .map_err(|_| ErrorKind::InvalidUIntLiteral)
            }
        } else {
            i32::from_str_radix(text, radix)
                .map(INT_CONST)
                .map_err(|_| ErrorKind::InvalidIntLiteral)
        }
    }

    pub(crate) fn parse_digits(text: &str) -> Self {
        use Token::*;

        let hex_prefix = text.strip_prefix("0x").or_else(|| text.strip_prefix("0X"));

        let result = if (text.ends_with('f')
            || text.ends_with('F')
            || text.contains('.')
            || (text.contains(['e', 'E'])))
            && hex_prefix.is_none()
        {
            // Floating-point constant

            if let Some(double) = text.strip_suffix("lf").or_else(|| text.strip_suffix("LF")) {
                double
                    .parse()
                    .map(DOUBLE_CONST)
                    .map_err(|_| ErrorKind::InvalidDoubleLiteral)
            } else if let Some(float) = text.strip_suffix(['f', 'F']).or(Some(text)) {
                float
                    .parse()
                    .map(FLOAT_CONST)
                    .map_err(|_| ErrorKind::InvalidFloatLiteral)
            } else {
                Err(ErrorKind::InvalidFloatLiteral)
            }
        } else {
            // Integer constant
            if let Some(text) = hex_prefix {
                // Hexadecimal constant
                Self::parse_int(text, 16)
            } else if let Some(text) = text.strip_prefix('0') {
                if text.is_empty() {
                    if Self::strip_suffix(text).0 {
                        Ok(UINT_CONST(0))
                    } else {
                        Ok(INT_CONST(0))
                    }
                } else {
                    // Octal constant
                    Self::parse_int(text, 8)
                }
            } else {
                // Decimal constant
                Self::parse_int(text, 10)
            }
        };

        match result {
            Ok(res) => res,
            Err(err) => ERROR(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Token::{self, *};

    #[test]
    fn test_parse_float_constant() {
        assert_eq!(Token::parse_digits("0."), FLOAT_CONST(0.));
        assert_eq!(Token::parse_digits(".0"), FLOAT_CONST(0.));
        assert_eq!(Token::parse_digits(".035"), FLOAT_CONST(0.035));
        assert_eq!(Token::parse_digits("0."), FLOAT_CONST(0.));
        assert_eq!(Token::parse_digits("0.035"), FLOAT_CONST(0.035));
        assert_eq!(Token::parse_digits(".035f"), FLOAT_CONST(0.035));
        assert_eq!(Token::parse_digits("0.f"), FLOAT_CONST(0.));
        assert_eq!(Token::parse_digits("314.f"), FLOAT_CONST(314.));
        assert_eq!(Token::parse_digits("0.035f"), FLOAT_CONST(0.035));
        assert_eq!(Token::parse_digits(".035F"), FLOAT_CONST(0.035));
        assert_eq!(Token::parse_digits("0.F"), FLOAT_CONST(0.));
        assert_eq!(Token::parse_digits("0.035F"), FLOAT_CONST(0.035));
        assert_eq!(Token::parse_digits("1.03e+34"), FLOAT_CONST(1.03e+34));
        assert_eq!(Token::parse_digits("1.03E+34"), FLOAT_CONST(1.03E+34));
        assert_eq!(Token::parse_digits("1.03e-34"), FLOAT_CONST(1.03e-34));
        assert_eq!(Token::parse_digits("1.03E-34"), FLOAT_CONST(1.03E-34));
        assert_eq!(Token::parse_digits("1.03e+34f"), FLOAT_CONST(1.03e+34));
        assert_eq!(Token::parse_digits("1.03E+34f"), FLOAT_CONST(1.03E+34));
        assert_eq!(Token::parse_digits("1.03e-34f"), FLOAT_CONST(1.03e-34));
        assert_eq!(Token::parse_digits("1.03E-34f"), FLOAT_CONST(1.03E-34));
        assert_eq!(Token::parse_digits("1.03e+34F"), FLOAT_CONST(1.03e+34));
        assert_eq!(Token::parse_digits("1.03E+34F"), FLOAT_CONST(1.03E+34));
        assert_eq!(Token::parse_digits("1.03e-34F"), FLOAT_CONST(1.03e-34));
        assert_eq!(Token::parse_digits("1.03E-34F"), FLOAT_CONST(1.03E-34));

        assert_eq!(Token::parse_digits("1e-34"), FLOAT_CONST(1E-34));
        assert_eq!(Token::parse_digits("1e-34f"), FLOAT_CONST(1E-34));
        assert_eq!(Token::parse_digits("1E-34f"), FLOAT_CONST(1E-34));
        assert_eq!(Token::parse_digits("1e-34F"), FLOAT_CONST(1E-34));
        assert_eq!(Token::parse_digits("1E-34F"), FLOAT_CONST(1E-34));
    }

    #[test]
    fn test_parse_double_constant() {
        assert_eq!(Token::parse_digits("0.lf"), DOUBLE_CONST(0.));
        assert_eq!(Token::parse_digits("0.035lf"), DOUBLE_CONST(0.035));
        assert_eq!(Token::parse_digits(".035lf"), DOUBLE_CONST(0.035));
        assert_eq!(Token::parse_digits(".035LF"), DOUBLE_CONST(0.035));
        assert_eq!(Token::parse_digits("0.LF"), DOUBLE_CONST(0.));
        assert_eq!(Token::parse_digits("0.035LF"), DOUBLE_CONST(0.035));
        assert_eq!(Token::parse_digits("1.03e+34lf"), DOUBLE_CONST(1.03e+34));
        assert_eq!(Token::parse_digits("1.03E+34lf"), DOUBLE_CONST(1.03E+34));
        assert_eq!(Token::parse_digits("1.03e-34lf"), DOUBLE_CONST(1.03e-34));
        assert_eq!(Token::parse_digits("1.03E-34lf"), DOUBLE_CONST(1.03E-34));
        assert_eq!(Token::parse_digits("1.03e+34LF"), DOUBLE_CONST(1.03e+34));
        assert_eq!(Token::parse_digits("1.03E+34LF"), DOUBLE_CONST(1.03E+34));
        assert_eq!(Token::parse_digits("1.03e-34LF"), DOUBLE_CONST(1.03e-34));
        assert_eq!(Token::parse_digits("1.03E-34LF"), DOUBLE_CONST(1.03E-34));
    }

    #[test]
    fn test_parse_int_constant() {
        assert_eq!(Token::parse_digits("0"), INT_CONST(0));
        assert_eq!(Token::parse_digits("012"), INT_CONST(0o12));
        assert_eq!(Token::parse_digits("03"), INT_CONST(0o3));
        assert_eq!(Token::parse_digits("07654321"), INT_CONST(0o7654321));
        assert_eq!(Token::parse_digits("076556"), INT_CONST(0o76556));
        assert_eq!(Token::parse_digits("0x0123789"), INT_CONST(0x0123789));
        assert_eq!(Token::parse_digits("0x3"), INT_CONST(0x3));
        assert_eq!(Token::parse_digits("0x9ABCDEF"), INT_CONST(0x9ABCDEF));
        assert_eq!(Token::parse_digits("0x9abcdef"), INT_CONST(0x9abcdef));
        assert_eq!(Token::parse_digits("0xABCDEF"), INT_CONST(0xabcdef));
        assert_eq!(Token::parse_digits("0xabcdef"), INT_CONST(0xabcdef));
        assert_eq!(Token::parse_digits("123456"), INT_CONST(123456));
        assert_eq!(Token::parse_digits("13"), INT_CONST(13));
        assert_eq!(Token::parse_digits("3"), INT_CONST(3));
        assert_eq!(Token::parse_digits("42"), INT_CONST(42));
    }

    #[test]
    fn test_parse_uint_constant() {
        assert_eq!(Token::parse_digits("0u"), UINT_CONST(0));
        assert_eq!(Token::parse_digits("1u"), UINT_CONST(1));
        assert_eq!(
            Token::parse_digits("0xffffffffU"),
            UINT_CONST(0xffffffffu32)
        );
    }
}
