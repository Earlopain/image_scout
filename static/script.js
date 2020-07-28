window.addEventListener("DOMContentLoaded", () => {
	const comparator = new Compare("image-orig", "image-juxtaposition", ".image-compare");
});


class Compare {

	constructor(originalImageId, targetId, compareSelector) {
		this.originalImage = document.getElementById(originalImageId);
		this.target = document.getElementById(targetId);
		const cropper = Jcrop.attach(originalImageId, { handles: ["sw", "nw", "ne", "se"], aspectRatio: this.originalImage.naturalWidth / this.originalImage.naturalHeight });

		cropper.listen("crop.change", async e => {
			this.lastEvent = e;
			this.createComparasion();
		})

		this.imageToCompareTo = document.querySelector(compareSelector);

		document.querySelectorAll(compareSelector).forEach(e => e.addEventListener("click", e => {
			this.imageToCompareTo = e.target;
			this.createComparasion();
		}));
	}

	async createComparasion() {
		if(this.imageToCompareTo && this.lastEvent) {
			const croppedOriginal = await this.getCroppedBlobUrl(this.lastEvent, this.originalImage);
			const croppedToCompare = await this.getCroppedBlobUrl(this.lastEvent, this.imageToCompareTo);
			this.createCompareJuxtapose(croppedOriginal, croppedToCompare);
		}
	}


	createCompareJuxtapose(origUrl, comapreToUrl) {
		const currentPercentage = this.currentSlider ? this.currentSlider.getPosition() : 50;
		document.getElementById(this.target.id).innerHTML = "";
		this.currentSlider = new juxtapose.JXSlider("#" + this.target.id,
			[
				{
					src: origUrl,
					label: "Uploaded",
				},
				{
					src: comapreToUrl,
					label: "Compare To",
				}
			],
			{
				showLabels: true,
				startingPosition: currentPercentage,
				makeResponsive: true
			});
	}

	async getCroppedBlobUrl(e, img) {
		const ratio = img.naturalHeight / img.height;
		const x = e.pos.x * ratio;
		const y = e.pos.y * ratio;
		const width = e.pos.w * ratio;
		const height = e.pos.h * ratio;

		const canvas = document.createElement("canvas");
		canvas.width = img.naturalWidth;
		canvas.height = img.naturalHeight;
		const ctx = canvas.getContext("2d");
		ctx.imageSmoothingEnabled = false;

		ctx.drawImage(img, this.normalize(x, this.originalImage.width, img.width), this.normalize(y, this.originalImage.height, img.height), this.normalize(width, this.originalImage.width, img.width), this.normalize(height, this.originalImage.height, img.height), 0, 0, img.naturalWidth, img.naturalHeight);
		return await this.canvasToBlobUrl(canvas);
	}

	canvasToBlobUrl(canvas) {
		return new Promise(resolve => {
			canvas.toBlob((blob, mimeType) => resolve(URL.createObjectURL(blob, { type: mimeType })));
		})
	}

	normalize(val, maxVal, newMax) {
		return val * newMax / maxVal;
	};
}
