import java.io._

object ProblemB {
  def main(args: Array[String]): Unit = processStdInOut()

  def processStdInOut(): Unit = {
    val src = io.Source.fromInputStream(System.in)
    try {
      val bw = new BufferedWriter(new OutputStreamWriter(System.out));
      try {
        val lines = src.getLines()
        processLines(lines, bw)
      } finally {
        bw.flush()
        bw.close()
      }
    } finally {
      src.close();
    }
  }

  // Standard code above, custom code below

  def processLines(lines: Iterator[String], bw: BufferedWriter): Unit = {
    val line = lines.next()
    val strings = line.split(' ')
    val n = strings(0).toInt
    val m = strings(1).toInt
    val table = Array.ofDim[Int](n, m)
    for (r <- 0 until n) {
      val row = lines.next()
      val entries = row.split(' ').map(_.toInt - 1)  // i.e. make zero-based
      table(r) = entries
    }

    val hasSolution = solve(n, m, table)
    if (hasSolution) { bw.write("YES") } else {bw.write("NO") }
    bw.newLine()
  }

  def solve(n: Int, m: Int, table: Array[Array[Int]]): Boolean = {
    val indexCombinations = (1 until m).flatMap { col => (0 until col).map( (_, col)) }

    def isRowSorted(row: Array[Int]): Boolean = row.zipWithIndex.forall(p => p._1 == p._2)

    def swapEntriesInPlace(row: Array[Int], swap: (Int, Int)): Unit = {
      val (col1, col2) = swap
      val firstEntry = row(col1)
      row(col1) = row(col2)
      row(col2) = firstEntry
    }

    def isSortedAfterSwaps(row: Array[Int], swaps: (Int, Int)* ): Boolean = {
      val newRow = row.clone()
      swaps.foreach(swapEntriesInPlace(newRow, _))
      isRowSorted(newRow)
    }

    def columnSwapHasSolution(colSwap: (Int, Int)): Boolean = {
      def rowCanBeOrdered(rowIndex: Int): Boolean = {
        val rowPrior = table(rowIndex)
        def entriesCanBeSwapped(entrySwap: (Int, Int)): Boolean = {
          // Either do the column swap before or after:
          isSortedAfterSwaps(rowPrior, colSwap, entrySwap) ||
            isSortedAfterSwaps(rowPrior, entrySwap, colSwap)
        }
        // See if column swap is sufficient to reorder, else try swapping each combination of entries...
        val rowHasSolution = isSortedAfterSwaps(rowPrior, colSwap) ||
          indexCombinations.exists(entriesCanBeSwapped(_))
        rowHasSolution
      }
      val hasSolution = (0 until n).forall(rowCanBeOrdered)
      hasSolution
    }

    def isThereASolutionWithNoColumnSwap(): Boolean = {
      def rowCanBeOrdered(rowIndex: Int): Boolean = {
        val rowPrior = table(rowIndex)
        // See if row is already sorted, else try swapping each combination of entries...
        val rowHasSolution = isRowSorted(rowPrior) || indexCombinations.exists(isSortedAfterSwaps(rowPrior, _))
        rowHasSolution
      }
      val hasSolution = (0 until n).forall(rowCanBeOrdered)
      hasSolution
    }

    isThereASolutionWithNoColumnSwap() || indexCombinations.exists(columnSwapHasSolution)
  }
}
