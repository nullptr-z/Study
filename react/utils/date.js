export function dateDiff(timestampDiff) {
  let computeTime = timestampDiff / 1000
	// let year = Math.floor(computeTime / 86400 / 365);
	// computeTime = computeTime % (86400 * 365);
	// let month = Math.floor(computeTime / 86400 / 30);
	computeTime = computeTime % (86400 * 30);
	let day = Math.floor(computeTime / 86400);

	computeTime = computeTime  % 86400;
  let hour = Math.floor(computeTime / 3600) + (day * 24);
	computeTime = computeTime  % 3600;
	let minute = Math.floor(computeTime / 60);
	computeTime  = computeTime % 60;
	let second = computeTime ;
  // year+'年,'+month+'月,'+day+'日,'+
  return hour + '时' + minute + '分' + second + '秒';
}