import java.io._
import scala.io.Source

import scala.annotation.tailrec

object ProblemA {
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

    val bumperCount = lines.next().toInt
    val bumpers = lines.next()
    val soln = solve(bumperCount, bumpers)
    bw.write(soln.toString)
    bw.newLine()
  }

  def solve(bumperCount: Int, bumpers: String): Int = {
    @tailrec
    def countLeft(pos: Int, count: Int): Int =
      if (pos == bumperCount || bumpers(pos) == '>') count
      else countLeft(pos + 1, count + 1)

    @tailrec
    def countRight(pos: Int, count: Int): Int =
      if (pos == -1 || bumpers(pos) == '<') count
      else countRight(pos - 1, count + 1)

    val totalCount = countLeft(0, 0) + countRight(bumperCount - 1, 0)
    totalCount
  }
}
