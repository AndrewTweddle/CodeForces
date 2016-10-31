import java.io._

import ProblemB.IndexedValue

import scala.io.Source
import scala.annotation.tailrec

object ProblemB {
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
  case class IndexedValue(index: Int, value: Int) extends Ordered[IndexedValue] {
    def compare(that: IndexedValue) = math.abs(this.value).compare(math.abs(that.value))
  }

  def processLines(lines: Iterator[String], bw: BufferedWriter): Unit = {
    val n = lines.next().toInt
    var minLMinusR = IndexedValue(0, 0)
    var maxLMinusR = IndexedValue(0, 0)
    var totalLMinusR = 0

    for (i <- 1 to n) {
      val column = lines.next.split(" ").map(_.toInt)
      val l = column(0)
      val r = column(1)
      val lMinusR = l - r
      totalLMinusR += lMinusR
      if (minLMinusR.value > lMinusR) minLMinusR = IndexedValue(i, lMinusR)
      if (maxLMinusR.value < lMinusR) maxLMinusR = IndexedValue(i, lMinusR)
    }

    val indexedVals = List(
      IndexedValue(0, totalLMinusR),  // Order important, so that changes aren't made unnecessarily
      IndexedValue(minLMinusR.index, totalLMinusR - 2 * minLMinusR.value),
      IndexedValue(maxLMinusR.index, totalLMinusR - 2 * maxLMinusR.value)
    )
    val bestIndVal = indexedVals.max
    bw.write(bestIndVal.index.toString)
    bw.newLine()
  }
}
