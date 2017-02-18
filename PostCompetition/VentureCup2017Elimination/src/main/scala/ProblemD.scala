import java.io._

import scala.annotation.tailrec
import scala.collection.mutable
import scala.io.Source

object ProblemD {
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
      src.close()
    }
  }

  // Standard code above, custom code below

  def processLines(lines: Iterator[String], bw: BufferedWriter): Unit = {
    val line = lines.next()
    val nk = line.split(" ").map(_.toInt)
    val (n, k) = (nk(0), nk(1))
    if (k > n / 2)
      writeSolution(bw, n, n - k)  // always cycle around in increasing order, as the algorithm assumes this
    else
      writeSolution(bw, n, k)
    bw.newLine()
  }

  def writeSolution(bw: BufferedWriter, n: Int, k: Int): Unit = {

    @tailrec
    def writeVertices(currTotal: Long, currCrossings: Long, currX: Long): Unit = {
      val newX = (currX + k - 1) % n + 1
      val crossingAdjustment = if (newX > 1 && newX <= k) 1 else 0
      val newCrossings = currCrossings + crossingAdjustment
      val newTotal = currTotal + 2 * newCrossings + 1 - crossingAdjustment
      bw.write(newTotal.toString)
      bw.write(" ")
      if (newX != 1) {
        writeVertices(newTotal, newCrossings, newX)
      }
    }

    writeVertices(1L, 0L, 1L)
  }

}
