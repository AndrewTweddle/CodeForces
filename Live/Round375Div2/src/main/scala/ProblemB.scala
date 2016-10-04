import scala.io._
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
  case class Solution(longestWordOutsideParens: Int, wordCountInParens: Int)

  def processLines(lines: Iterator[String], bw: BufferedWriter): Unit = {
    val length = lines.next().toInt
    val str = lines.next() + '_';  // To make it easier to not miss the final word
    val solution = solve(str.toList)
    bw.write(s"${solution.longestWordOutsideParens} ${solution.wordCountInParens}")
    bw.newLine()
  }

  case class Accumulator(currWordLength: Int, openParensCount: Int,
                         longestWordOutsideParens: Int, wordCountInParens: Int)

  def solve(str: List[Char]): Solution = {
    def charToParensCountDelta(ch: Char) = ch match {
      case '(' => 1
      case ')' => -1
      case _ => 0
    }

    val accumulator: Accumulator = str.foldLeft(Accumulator(0, 0, 0, 0)) {
      (acc: Accumulator, ch: Char) => ch match {
        // Outside parentheses...
        case '_' | '(' if acc.openParensCount == 0 =>  // end of current word
          acc.copy(
            currWordLength = 0,
            longestWordOutsideParens = math.max(acc.longestWordOutsideParens, acc.currWordLength),
            openParensCount = charToParensCountDelta(ch)
          )

        case _ if acc.openParensCount == 0 => acc.copy(acc.currWordLength + 1)

        // Inside parentheses...

        case '_' | '(' | ')' => {
          // end of current word
          val newWordCountInParens = acc.wordCountInParens + (if (acc.currWordLength == 0) 0 else 1)
          acc.copy(
            currWordLength = 0,
            wordCountInParens = newWordCountInParens,
            openParensCount = acc.openParensCount + charToParensCountDelta(ch)
          )
        }

        case _ => acc.copy(acc.currWordLength + 1)
      }
    }
    Solution(accumulator.longestWordOutsideParens, accumulator.wordCountInParens)
  }
}
