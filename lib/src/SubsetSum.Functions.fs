module Functions.SubsetSum

open Domain.SubsetSum

[<Literal>]
let FloatCompTolerance = 1e-9

let isAlmostZero fl = abs (fl - 0.0) <= FloatCompTolerance

let floatsAlmostEqual f1 f2 =
    abs (abs (f1 - f2) - 0.0) <= FloatCompTolerance

let subsetSum: SubsetSum =
    fun input ->
        let { Input.Elements = elements
              Input.Target = target } =
            input

        let rec combinations (elems: Element seq) =
            seq {
                if Seq.isEmpty elems then
                    yield Seq.empty
                else
                    let head = Seq.head elems
                    let tail = Seq.tail elems

                    for c in combinations tail do
                        yield c
                        yield Seq.append (seq [ head ]) c
            }

        let validCombinations =
            combinations elements
            |> Seq.filter (fun c ->
                Seq.sumBy (fun e -> e.Value) c
                |> floatsAlmostEqual target)

        match validCombinations |> Seq.tryHead with
        | Some validCombination ->
            Ok
                { Elements = validCombination
                  Target = target }
        | None -> Error(NoSolution(NoSolutionError "No solution."))
