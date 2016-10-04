import scala.io._
import java.io._

object ProblemA {
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
    val strings = line.split(" ")
    val points = strings.map(_.toInt).toIndexedSeq
    val solution = solve(points)
    bw.write(s"$solution")
    bw.newLine()
  }

  def solve(points: IndexedSeq[Int]): Int = {
    val sortedPoints = points.sorted
    val median = sortedPoints(1) // median
    val totalDistance = sortedPoints.map(pos => math.abs(pos - median)).sum
    totalDistance
  }
}
