module Functions.ExcelInput

// NuGet packages
open ClosedXML.Excel

// types
open Domain.ExcelInput
open Domain.SubsetSum

[<Literal>]
let private LabelColName = "Label"

[<Literal>]
let private ValueColName = "Value"

[<Literal>]
let private TargetsColName = "Targets"

let private (>>=) = Result.bind

type private Header =
    | Label
    | Value
    | Target

let private headerToStr header =
    function
    | Label -> "label"
    | Value -> "value"
    | Target -> "targets"

/// Generic function for retrieving and type-casting a cell's value
let private getValue<'T> (fromCell: IXLCell) = fromCell.GetValue<'T>()

let private getStringValue = getValue<string>
let private getFloatValue = getValue<float>

/// Tries to extract the columns from the provided sheet, failing if there are less than 3 used columns
let private sheetToCols (sheet: IXLWorksheet) =
    let cols = sheet.ColumnsUsed()

    if Seq.length cols < 3 then
        Error
            "There were too few (less than 3) columns in the provided workbook sheet. There must be a column for Label, Value, and Targets."
    else
        Ok cols

/// Gets the header value (first row of a column) as a string
let private getHeaderValue (column: IXLColumn) =
    column.FirstCellUsed() |> getStringValue

/// Tries to find a specific column by name (case insensitive) from a sequence of columns
let private tryFindColumn (columnHeader: string) (columns: IXLColumns) =
    let colMatches =
        columns
        |> Seq.filter (fun x -> (getHeaderValue x).ToLowerInvariant() = columnHeader.ToLowerInvariant())

    if Seq.isEmpty colMatches then
        Error $"No column matched the header \"{columnHeader}\""
    else
        Ok(Seq.head colMatches)

/// Tries to extract all cells after the header row of a given column
let private getValueCellsFromColumn (column: IXLColumn) =
    let valueCells = column.CellsUsed() |> Seq.tail

    if Seq.isEmpty valueCells then
        Error "No cells were defined after the header row for the provided column."
    else
        Ok valueCells


let readInput: ReadInput =
    fun fileReader filepath ->
        use fileStream = fileReader filepath

        use spreadsheet = new XLWorkbook(fileStream)
        let sheet1 = spreadsheet.Worksheets |> Seq.head



        failwith "not implemented"
