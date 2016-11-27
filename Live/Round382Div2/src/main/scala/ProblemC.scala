import java.io._
import scala.io.Source

import scala.annotation.tailrec

object ProblemC {
  def main(args: Array[String]): Unit = processStdInOut()

  def processStdInOut(): Unit = {
    val src = Source.fromInputStream(System.in)
    processFromSource(src)
  }

  def processFromSource(src: Source): Unit = {
    try {
      val bw = new BufferedWriter(new OutputStreamWriter(System.out));
      try {
        val lines = src.getLines()
        processLines(lines, bw)
      } finally {
        bw.flush()
      }
    } finally {
      src.close();
    }
  }

  // Standard code above, custom code below
  def processLines(lines: Iterator[String], bw: BufferedWriter): Unit = {
    val n = lines.next().toDouble
    val log2 = math.log(2)
    val rounds = math.ceil(math.log(n) / log2).toLong
    val adjRounds = if (math.pow(2, rounds) < n) rounds + 1
    else if (math.pow(2, rounds - 1) >= n) rounds - 1
    else rounds
    bw.write(adjRounds.toString)
    bw.newLine()
  }
}
