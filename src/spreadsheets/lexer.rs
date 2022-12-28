/*
 * The lexer is responsible for reading the formulas and splitting it into tokens.
 */

///// DRAFT !!! //////

/*
#[derive(Debug, Clone)]
pub enum Symbol {
    // !label Columns can have labels, which allows this ability to have different column groups in the same file as long as the number of columns stays consistent
    Label, // !label

    // @label<n> References a specific labeled column and a specific row n under that column relative to where the column was labeled. This is a reference operator with relative row traversal
    LabelReference, // @label<n>

    // (A..Z)^ copies the evaluated result of the cell above in the same column
    CopyAboveResult, // (A..Z)^ ex: A^ (without v)

    // ^^ Copies the formula from the cell above in the same column, with some special evaluation rules
    CopyAboveFormula, // ^^

    // (A..Z)n references a cell by a combination of a column-letter+row-number. Ex: A2 B3
    RowColumnReference, // (A..Z)n ex A1

    // (A..Z)^v copies the evaluated result of the last cell in the specified column from the most recently available column group that has data in that specified column
    CopyLastResult, // (A..Z)^v ex: A^v or B^v (with v)
}

pub enum Operations {
    // + Adds two numbers
    Add,

    // - Subtracts two numbers
    Subtract,

    // * Multiplies two numbers
    Multiply,

    // / Divides two numbers
    Divide,

    // % Modulo two numbers
    Modulo,

    // ^ Exponent two numbers
    Exponent,

    // & Concatenates two strings
    Concatenate,

    //  Splits two strings
    Split,

    // < Less than two numbers
    LessThan,

    // > Greater than two numbers
    GreaterThan,

    // <= Less than or equal to two numbers
    LessThanOrEqual,

    // >= Greater than or equal to two numbers
    GreaterThanOrEqual,

    // == Equal to two numbers
    EqualTo,

    // != Not equal to two numbers
    NotEqualTo,

    // && Logical AND two booleans
    LogicalAnd,

    // || Logical OR two booleans
    LogicalOr,

    // ! Logical NOT a boolean
    LogicalNot,
}
*/
