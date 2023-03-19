module Functions.SubsetSum

open Domain.SubsetSum

[<Literal>]
let FloatCompTolerance = 1e-9

let isAlmostZero fl = abs (fl - 0.0) <= FloatCompTolerance


let subsetSum: SubsetSum =
    fun input ->
        let { Input.Elements = elements
              Input.Target = target } =
            input

        let rec subsetSum' (elements': Element seq) (current: Element seq) (target': float) =
            // if the cumulative target (subtracts the head each iteration)
            // is zero, we've found a solution so we can return it.
            if target' |> isAlmostZero then
                Ok { Elements = current; Target = target }
            // otherwise, if the sequence is empty and the cumulative
            // target isn't zero, we've exhausted all options, meaning
            // there's no solution
            elif Seq.isEmpty elements' then
                "No solution."
                |> Error
                |> Result.mapError NoSolutionError
                |> Result.mapError NoSolution
            // Otherwise, if target' != 0 and there are still elements,
            // we can pop off the head, include it in the possible solution,
            // subtract it from the cumulative target, and iterate.
            else
                let head = Seq.head elements'
                let tail = Seq.tail elements'
                let targetValue = target'
                let headValue = head.Value
                let newCurrent = Seq.append current (Seq.ofList [ head ])

                match subsetSum' tail newCurrent (targetValue - headValue) with
                | Ok output -> Ok output // if we get OK, that means we've gotten a terminal solution
                | Error _ -> subsetSum' tail current targetValue // if we get error, that means we should try again without including this iteration's head

        subsetSum' elements Seq.empty<Element> target
