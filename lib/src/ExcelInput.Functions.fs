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
let private sheetToCols (sheet: IXLWorksheet) = sheet.ColumnsUsed()

/// Gets the header value (first row of a column) as a string
let private getHeaderValue (column: IXLColumn) =
    column.FirstCellUsed() |> getStringValue

/// Determines if a column's header matches a string
let private headerMatchesString (strVal: string) (column: IXLColumn) =
    column
    |> getHeaderValue
    |> (fun x -> x.ToLowerInvariant() = strVal.ToLowerInvariant())

/// Tries to find a specific column by name (case insensitive) from a sequence of columns
let private findColumn (columnHeader: string) (columns: IXLColumns) =
    let colMatches =
        columns
        |> Seq.filter (headerMatchesString columnHeader)

    if Seq.isEmpty colMatches then
        failwith $"No column matched the header \"{columnHeader}\""
    else
        Seq.head colMatches

/// Tries to extract all cells after the header row of a given column
let private getValueCellsFromColumn (column: IXLColumn) =
    let valueCells = column.CellsUsed() |> Seq.tail

    if Seq.isEmpty valueCells then
        failwith "No cells were defined after the header row for the provided column."
    else
        valueCells

let readInput: ReadInput =
    fun fileReader filepath ->
        use fileStream = fileReader filepath

        use spreadsheet = new XLWorkbook(fileStream)
        let sheet1 = spreadsheet.Worksheets |> Seq.head
        let columns = sheet1 |> sheetToCols

        let labelColumn =
            columns
            |> findColumn "label"
            |> getValueCellsFromColumn
            |> Seq.map getStringValue
            |> List.ofSeq

        let valueColumn =
            columns
            |> findColumn "value"
            |> getValueCellsFromColumn
            |> Seq.map getFloatValue
            |> List.ofSeq

        let targetColumn =
            columns
            |> findColumn "targets"
            |> getValueCellsFromColumn
            |> Seq.map getFloatValue
            |> List.ofSeq

        let inputElems =
            Seq.zip labelColumn valueColumn
            |> Seq.map (fun (l, v) -> { Label = l; Value = v })
            |> List.ofSeq

        { InputElements = inputElems
          Targets = targetColumn }
        |> Ok
