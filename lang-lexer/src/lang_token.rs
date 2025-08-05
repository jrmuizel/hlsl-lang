use hlsl_lang_pp::types;

use hlsl_lang_types::ast::NodeSpan;
use lang_util::position::LexerPosition;

use crate::{ParseContext, Token};

pub fn lang_token(
    ctx: &ParseContext,
    text: &str,
    pos: NodeSpan,
    token_kind: types::Token,
) -> Result<(LexerPosition, Token, LexerPosition), (types::Token, types::token::ErrorKind)> {
    Ok((
        pos.start(),
        match token_kind {
            types::Token::IDENT(ident) => {
                if ctx.is_type_name(&ident) {
                    Token::TypeName(ident)
                } else {
                    // It is an identifier
                    Token::Identifier(ident)
                }
            }
            types::Token::TYPE_NAME(type_name) => match type_name {
                types::TypeName::VOID => Token::Void,
                types::TypeName::INT => Token::Int,
                types::TypeName::BOOL => Token::Bool,
                types::TypeName::FLOAT => Token::Float,
                types::TypeName::DOUBLE => Token::Double,
                // HLSL vector types (enum variants use GLSL-style names internally)
                types::TypeName::VEC2 => Token::Vec2,
                types::TypeName::VEC3 => Token::Vec3,
                types::TypeName::VEC4 => Token::Vec4,
                types::TypeName::IVEC2 => Token::IVec2,
                types::TypeName::IVEC3 => Token::IVec3,
                types::TypeName::IVEC4 => Token::IVec4,
                types::TypeName::BVEC2 => Token::BVec2,
                types::TypeName::BVEC3 => Token::BVec3,
                types::TypeName::BVEC4 => Token::BVec4,
                types::TypeName::UINT => Token::UInt,
                types::TypeName::UVEC2 => Token::UVec2,
                types::TypeName::UVEC3 => Token::UVec3,
                types::TypeName::UVEC4 => Token::UVec4,
                // HLSL matrix types (enum variants use GLSL-style names internally)
                types::TypeName::MAT2 => Token::Mat2,
                types::TypeName::MAT3 => Token::Mat3,
                types::TypeName::MAT4 => Token::Mat4,
                types::TypeName::MAT2X2 => Token::Mat22,
                types::TypeName::MAT2X3 => Token::Mat2x3,
                types::TypeName::MAT2X4 => Token::Mat2x4,
                types::TypeName::MAT3X2 => Token::Mat3x2,
                types::TypeName::MAT3X3 => Token::Mat33,
                types::TypeName::MAT3X4 => Token::Mat3x4,
                types::TypeName::MAT4X2 => Token::Mat4x2,
                types::TypeName::MAT4X3 => Token::Mat4x3,
                types::TypeName::MAT4X4 => Token::Mat44,
                types::TypeName::MAT1X1 => Token::Mat22, // Map to closest equivalent
                types::TypeName::MAT1X2 => Token::Mat2x3, // Map to closest equivalent
                types::TypeName::MAT1X3 => Token::Mat2x3, // Map to closest equivalent
                types::TypeName::MAT1X4 => Token::Mat2x4, // Map to closest equivalent
                types::TypeName::MAT2X1 => Token::Mat3x2, // Map to closest equivalent
                types::TypeName::MAT3X1 => Token::Mat3x2, // Map to closest equivalent
                types::TypeName::MAT4X1 => Token::Mat4x2, // Map to closest equivalent
                // Basic HLSL sampler type
                types::TypeName::SAMPLER => Token::Sampler,
                // HLSL texture types
                types::TypeName::TEXTURE_1D => Token::Texture1D,
                types::TypeName::TEXTURE_2D => Token::Texture2D,
                types::TypeName::TEXTURE_3D => Token::Texture3D,
                types::TypeName::TEXTURE_CUBE => Token::TextureCube,
                types::TypeName::TEXTURE_1D_ARRAY => Token::Texture1DArray,
                types::TypeName::TEXTURE_2D_ARRAY => Token::Texture2DArray,
                types::TypeName::TEXTURE_CUBE_ARRAY => Token::TextureCubeArray,
                types::TypeName::TEXTURE_2D_MS => Token::Texture2DMS,
                types::TypeName::TEXTURE_2D_MS_ARRAY => Token::Texture2DMSArray,
                types::TypeName::TEXTURE_2D_RECT => Token::Texture2DRect,
                types::TypeName::TEXTURE_BUFFER => Token::TextureBuffer,
                types::TypeName::STRUCTUREDBUFFER => Token::StructuredBuffer,
                types::TypeName::RWSTRUCTUREDBUFFER => Token::RWStructuredBuffer,
                other => Token::TypeName(other.to_string().into()),
            },
            types::Token::FLOAT_CONST(val) => Token::FloatConstant(val),
            types::Token::INT_CONST(val) => Token::IntConstant(val),
            types::Token::UINT_CONST(val) => Token::UIntConstant(val),
            types::Token::BOOL_CONST(val) => Token::BoolConstant(val),
            types::Token::DOUBLE_CONST(val) => Token::DoubleConstant(val),
            types::Token::STRING_CONST(val) => Token::StringConstant(val),
            types::Token::LEFT_OP => Token::LeftOp,
            types::Token::RIGHT_OP => Token::RightOp,
            types::Token::INC_OP => Token::IncOp,
            types::Token::DEC_OP => Token::DecOp,
            types::Token::LE_OP => Token::LeOp,
            types::Token::GE_OP => Token::GeOp,
            types::Token::EQ_OP => Token::EqOp,
            types::Token::NE_OP => Token::NeOp,
            types::Token::AND_OP => Token::AndOp,
            types::Token::OR_OP => Token::OrOp,
            types::Token::XOR_OP => Token::XorOp,
            types::Token::MUL_ASSIGN => Token::MulAssign,
            types::Token::DIV_ASSIGN => Token::DivAssign,
            types::Token::ADD_ASSIGN => Token::AddAssign,
            types::Token::MOD_ASSIGN => Token::ModAssign,
            types::Token::LEFT_ASSIGN => Token::LeftAssign,
            types::Token::RIGHT_ASSIGN => Token::RightAssign,
            types::Token::AND_ASSIGN => Token::AndAssign,
            types::Token::XOR_ASSIGN => Token::XorAssign,
            types::Token::OR_ASSIGN => Token::OrAssign,
            types::Token::SUB_ASSIGN => Token::SubAssign,
            types::Token::LPAREN => Token::LeftParen,
            types::Token::RPAREN => Token::RightParen,
            types::Token::LBRACKET => Token::LeftBracket,
            types::Token::RBRACKET => Token::RightBracket,
            types::Token::LBRACE => Token::LeftBrace,
            types::Token::RBRACE => Token::RightBrace,
            types::Token::PERIOD => Token::Dot,
            types::Token::COMMA => Token::Comma,
            types::Token::COLON => Token::Colon,
            types::Token::EQUAL => Token::Equal,
            types::Token::SEMICOLON => Token::Semicolon,
            types::Token::BANG => Token::Bang,
            types::Token::DASH => Token::Dash,
            types::Token::TILDE => Token::Tilde,
            types::Token::PLUS => Token::Plus,
            types::Token::ASTERISK => Token::Star,
            types::Token::SLASH => Token::Slash,
            types::Token::PERCENT => Token::Percent,
            types::Token::LANGLE => Token::LeftAngle,
            types::Token::RANGLE => Token::RightAngle,
            types::Token::BAR => Token::VerticalBar,
            types::Token::CARET => Token::Caret,
            types::Token::AMPERSAND => Token::Ampersand,
            types::Token::QUESTION => Token::Question,
            types::Token::HASH => {
                return Err((token_kind, types::token::ErrorKind::InvalidToken));
            }
            types::Token::CONST => Token::Const,
            types::Token::STATIC => Token::Static,
            types::Token::EXTERN => Token::Extern,
            types::Token::INLINE => Token::Inline,
            types::Token::UNIFORM => Token::Uniform,
            types::Token::SHARED => Token::Shared,
            types::Token::COHERENT => Token::Coherent,
            types::Token::VOLATILE => Token::Volatile,
            types::Token::RESTRICT => Token::Restrict,
            types::Token::READONLY => Token::ReadOnly,
            types::Token::WRITEONLY => Token::WriteOnly,
            types::Token::REGISTER => Token::Register,
            types::Token::LAYOUT => Token::Layout,
            types::Token::CENTROID => Token::Centroid,
            types::Token::FLAT => Token::Flat,
            types::Token::SMOOTH => Token::Smooth,
            types::Token::NOPERSPECTIVE => Token::NoPerspective,
            types::Token::PATCH => Token::Patch,
            types::Token::SAMPLE => Token::Sample,
            types::Token::INVARIANT => Token::Invariant,
            types::Token::PRECISE => Token::Precise,
            types::Token::BREAK => Token::Break,
            types::Token::CONTINUE => Token::Continue,
            types::Token::DO => Token::Do,
            types::Token::FOR => Token::For,
            types::Token::WHILE => Token::While,
            types::Token::SWITCH => Token::Switch,
            types::Token::CASE => Token::Case,
            types::Token::DEFAULT => Token::Default,
            types::Token::IF => Token::If,
            types::Token::ELSE => Token::Else,
            types::Token::SUBROUTINE => Token::Subroutine,
            types::Token::IN => Token::In,
            types::Token::OUT => Token::Out,
            types::Token::INOUT => Token::InOut,
            types::Token::DISCARD => Token::Discard,
            types::Token::RETURN => Token::Return,
            types::Token::LOWP => Token::LowPrecision,
            types::Token::MEDIUMP => Token::MediumPrecision,
            types::Token::HIGHP => Token::HighPrecision,
            types::Token::PRECISION => Token::Precision,
            types::Token::STRUCT => Token::Struct,
            types::Token::CBUFFER => Token::CBuffer,
            types::Token::SAMPLER_STATE => Token::SamplerState,
            types::Token::SAMPLER_COMPARISON_STATE => Token::SamplerComparisonState,
            types::Token::ATTRIBUTE => Token::Attribute,
            types::Token::VARYING => Token::Varying,
            types::Token::COMMON
            | types::Token::PARTITION
            | types::Token::ACTIVE
            | types::Token::ASM
            | types::Token::CLASS
            | types::Token::UNION
            | types::Token::ENUM
            | types::Token::TYPEDEF
            | types::Token::TEMPLATE
            | types::Token::THIS
            | types::Token::RESOURCE
            | types::Token::GOTO
            | types::Token::NOINLINE
            | types::Token::PUBLIC
            | types::Token::EXTERNAL
            | types::Token::INTERFACE
            | types::Token::LONG
            | types::Token::SHORT
            | types::Token::HALF
            | types::Token::FIXED
            | types::Token::UNSIGNED
            | types::Token::SUPERP
            | types::Token::OUTPUT
            | types::Token::FILTER
            | types::Token::SIZEOF
            | types::Token::CAST
            | types::Token::NAMESPACE
            | types::Token::USING => {
                return Err((token_kind, types::token::ErrorKind::InvalidToken));
            }
            types::Token::WS => Token::Whitespace,
            types::Token::COMMENT => {
                if text.starts_with("//") {
                    Token::SingleLineComment
                } else {
                    Token::MultiLineComment
                }
            }
            types::Token::ERROR(kind) => {
                return Err((token_kind, kind));
            }
        },
        pos.end(),
    ))
}
