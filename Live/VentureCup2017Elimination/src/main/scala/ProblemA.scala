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
    val n = lines.next().toInt
    val soln = solve(n)
    bw.write(soln.toString)
    bw.newLine()
  }

  def solve(n: Int): Int = {
    def isComposite(i: Int): Boolean = {
      val maxDiv = (math.sqrt(i) + 0.5).toInt
      2.to(maxDiv).exists(i % _ == 0)
    }

    val mOpt = (1 to 1000).find(m => isComposite(m * n + 1))
    mOpt.get
  }
}
