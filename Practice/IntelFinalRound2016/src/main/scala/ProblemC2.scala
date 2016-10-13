import java.io._
import scala.io.Source
import scala.annotation.tailrec
import scala.reflect.ClassTag

object ProblemC2 {
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

  case class Point(val x: Int, val y: Int)

  def processLines(lines: Iterator[String], bw: BufferedWriter): Unit = {
    val line = lines.next()
    val strings = line.split(' ')
    val n = strings(0).toInt
    val m = strings(1).toInt
    val k = strings(2).toInt
    val signals = Array.ofDim[Point](k)
    for (i <- 0 until k) {
      val row = lines.next()
      val coords = row.split(' ').map(_.toInt)
      signals(i) = Point(coords(0), coords(1))
    }

    val firstTimeEachSignalIsHit = solve(n, m, k, signals)
    firstTimeEachSignalIsHit.map {
      case Some(tme) => tme.toString
      case _ => "-1"
    }.foreach { str =>
      bw.write(str)
      bw.newLine()
    }
  }

  sealed trait Direction { def inverse: Direction }
  final object NorthWest extends Direction { override val inverse = SouthEast }
  final object SouthWest extends Direction { override val inverse = NorthEast }
  final object NorthEast extends Direction { override val inverse = SouthWest }
  final object SouthEast extends Direction { override val inverse = NorthWest }

  class Border[T:ClassTag](val rowCount: Int, val columnCount: Int, initialValue: T)
  {
    val pointCount = 2 * (rowCount + columnCount)
    private val points: Array[T] = Array.fill[T](pointCount)(initialValue)
    def get(edgePoint: Point): T = points(pointToIndex(edgePoint))
    def set(edgePoint: Point, value: T): Unit = { points(pointToIndex(edgePoint)) = value }
    def pointToIndex(edgePoint: Point): Int = edgePoint match {
      case Point(x, 0) => x
      case Point(x, y) if x == columnCount => rowCount + y
      case Point(x, y) if y == rowCount    => 2 * rowCount + columnCount - x
      case Point(0, y) => 2 * (rowCount + columnCount) - y
      case _ => throw new IllegalArgumentException(s"Point (${edgePoint.x}, ${edgePoint.y}) is not a border point")
    }
    def isCornerPoint(point: Point): Boolean = point match {
      case Point(0, 0) => true
      case Point(0, y) if y == rowCount => true
      case Point(x, 0) if x == columnCount => true
      case Point(x, y) if x == columnCount && y == rowCount => true
      case _ => false
    }
  }
  case class Arrow(val position: Point, val direction: Direction, val timeSteps: Long) {
    def +(other: Arrow): Arrow = Arrow(other.position, other.direction, this.timeSteps + other.timeSteps)
  }
  case class SignalFromEdge(val index: Int, arrowToSignal: Arrow)

  def solve(n: Int, m: Int, k: Int, signals: Array[Point]): Array[Option[Long]] = {
    // Take a starting point and direction, and project to an edge of the n x m box.
    // Return a new arrow with its start point as the end point of the last arrow,
    // its direction the reflection of the previous arrow, and with timeSteps
    // based on how long previous arrow travelled for to reach the start of this arrow:
    def projectToBorder(startArrow: Arrow): Arrow = startArrow match {
      case Arrow(Point(x, y), NorthEast, _) if y >= x     => Arrow(Point(x + m - y, m), SouthEast, m-y)
      case Arrow(Point(x, y), NorthEast, _)               => Arrow(Point(n, y + n - x), NorthWest, n - x)
      case Arrow(Point(x, y), NorthWest, _) if x + y <= m => Arrow(Point(0, y + x), NorthEast, x)
      case Arrow(Point(x, y), NorthWest, _)               => Arrow(Point(x + y - m, m), SouthWest, m - y)
      case Arrow(Point(x, y), SouthEast, _) if x + y <= n => Arrow(Point(x + y, 0), NorthEast, y)
      case Arrow(Point(x, y), SouthEast, _)               => Arrow(Point(n, x + y - n), SouthWest, n - x)
      case Arrow(Point(x, y), SouthWest, _) if y <= x     => Arrow(Point(x - y, 0), NorthWest, y)
      case Arrow(Point(x, y), SouthWest, _)               => Arrow(Point(0, y - x), SouthEast, x)
    }

    val signalTimes = Array.fill[Option[Long]](k)(None)
    val diagonals = new Border[List[SignalFromEdge]](m, n, List.empty[SignalFromEdge])

    // Add signals along each diagonal by point on the border:
    val signalsFromEdges = for {
      i <- 0 until k
      dir <- List(NorthEast, NorthWest, SouthEast, SouthWest)
    } yield SignalFromEdge(i, projectToBorder(Arrow(signals(i), dir, 0)).copy(direction = dir.inverse))

    for (sig <- signalsFromEdges) {
      val pos = sig.arrowToSignal.position
      diagonals.set(pos, sig :: diagonals.get(pos))
    }

    @tailrec
    def projectRay(acc: Arrow): Unit = {
      // Update distances to signals along the current diagonal:
      val signalsOnDiagonal = diagonals.get(acc.position).filter(_.arrowToSignal.direction == acc.direction)
      for (sig <- signalsOnDiagonal) {
        if (signalTimes(sig.index) == None) {
          val timeToSignal = (acc + sig.arrowToSignal).timeSteps
          signalTimes(sig.index) = Some(timeToSignal)
        }
      }
      // Determine new position:
      val newAcc = acc + projectToBorder(acc)
      if (!diagonals.isCornerPoint(newAcc.position)) projectRay(newAcc)
    }

    projectRay(Arrow(Point(0, 0), NorthEast, 0))
    signalTimes
  }
}
